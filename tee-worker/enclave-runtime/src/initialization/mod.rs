/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

pub mod global_components;
pub mod parentchain;
use crate::{
	error::{Error, Result as EnclaveResult},
	initialization::global_components::{
		EnclaveBlockImportConfirmationHandler, EnclaveGetterExecutor, EnclaveLightClientSeal,
		EnclaveOCallApi, EnclaveRpcResponder, EnclaveShieldingKeyRepository, EnclaveSidechainApi,
		EnclaveSidechainBlockImportQueue, EnclaveSidechainBlockImportQueueWorker,
		EnclaveSidechainBlockImporter, EnclaveSidechainBlockSyncer, EnclaveStateFileIo,
		EnclaveStateHandler, EnclaveStateInitializer, EnclaveStateObserver,
		EnclaveStateSnapshotRepository, EnclaveStfEnclaveSigner, EnclaveTopPool,
		EnclaveTopPoolAuthor, DIRECT_RPC_REQUEST_SINK_COMPONENT, GLOBAL_ASSERTION_REPOSITORY,
		GLOBAL_ATTESTATION_HANDLER_COMPONENT, GLOBAL_DATA_PROVIDER_CONFIG,
		GLOBAL_DIRECT_RPC_BROADCASTER_COMPONENT, GLOBAL_INTEGRITEE_PARENTCHAIN_LIGHT_CLIENT_SEAL,
		GLOBAL_OCALL_API_COMPONENT, GLOBAL_RPC_WS_HANDLER_COMPONENT,
		GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT, GLOBAL_SIDECHAIN_BLOCK_COMPOSER_COMPONENT,
		GLOBAL_SIDECHAIN_BLOCK_SYNCER_COMPONENT, GLOBAL_SIDECHAIN_FAIL_SLOT_ON_DEMAND_COMPONENT,
		GLOBAL_SIDECHAIN_IMPORT_QUEUE_COMPONENT, GLOBAL_SIDECHAIN_IMPORT_QUEUE_WORKER_COMPONENT,
		GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT, GLOBAL_STATE_HANDLER_COMPONENT,
		GLOBAL_STATE_KEY_REPOSITORY_COMPONENT, GLOBAL_STATE_OBSERVER_COMPONENT,
		GLOBAL_TARGET_A_PARENTCHAIN_LIGHT_CLIENT_SEAL,
		GLOBAL_TARGET_B_PARENTCHAIN_LIGHT_CLIENT_SEAL, GLOBAL_TOP_POOL_AUTHOR_COMPONENT,
		GLOBAL_WEB_SOCKET_SERVER_COMPONENT,
	},
	ocall::OcallApi,
	rpc::{rpc_response_channel::RpcResponseChannel, worker_api_direct::public_api_rpc_handler},
	utils::{
		get_extrinsic_factory_from_integritee_solo_or_parachain,
		get_node_metadata_repository_from_integritee_solo_or_parachain,
		get_triggered_dispatcher_from_integritee_solo_or_parachain,
		get_validator_accessor_from_integritee_solo_or_parachain,
	},
	Hash,
};
use base58::ToBase58;
use codec::Encode;
use core::str::FromStr;
use ita_stf::{Getter, TrustedCallSigned};
use itc_direct_rpc_server::{
	create_determine_watch, rpc_connection_registry::ConnectionRegistry,
	rpc_ws_handler::RpcWsHandler,
};
use itc_peer_top_broadcaster::init;
use itc_tls_websocket_server::{
	certificate_generation::ed25519_self_signed_certificate,
	config_provider::FromFileConfigProvider, ws_server::TungsteniteWsServer, ConnectionToken,
	WebSocketServer,
};
use itp_attestation_handler::{AttestationHandler, IntelAttestationHandler};
use itp_component_container::{ComponentGetter, ComponentInitializer};
use itp_primitives_cache::GLOBAL_PRIMITIVES_CACHE;
use itp_settings::files::{
	ASSERTIONS_FILE, LITENTRY_PARENTCHAIN_LIGHT_CLIENT_DB_PATH, STATE_SNAPSHOTS_CACHE_SIZE,
	TARGET_A_PARENTCHAIN_LIGHT_CLIENT_DB_PATH, TARGET_B_PARENTCHAIN_LIGHT_CLIENT_DB_PATH,
};
use itp_sgx_crypto::{
	get_aes_repository, get_ed25519_repository, get_rsa3072_repository, key_repository::AccessKey,
};
use itp_stf_state_handler::{
	file_io::StateDir, handle_state::HandleState, query_shard_state::QueryShardState,
	state_snapshot_repository::VersionedStateAccess,
	state_snapshot_repository_loader::StateSnapshotRepositoryLoader, StateHandler,
};
use itp_top_pool::pool::Options as PoolOptions;
use itp_top_pool_author::author::{AuthorTopFilter, BroadcastedTopFilter};
use itp_types::{parentchain::ParentchainId, ShardIdentifier};
use its_sidechain::{
	block_composer::BlockComposer,
	slots::{FailSlotMode, FailSlotOnDemand},
};
use lc_data_providers::DataProviderConfig;
use lc_evm_dynamic_assertions::repository::EvmAssertionRepository;
use lc_scheduled_enclave::{ScheduledEnclaveUpdater, GLOBAL_SCHEDULED_ENCLAVE};
use lc_stf_task_receiver::{run_stf_task_receiver, StfTaskContext};
use lc_vc_task_receiver::run_vc_handler_runner;
use litentry_primitives::BroadcastedRequest;
use log::*;
use sgx_types::sgx_status_t;
use sp_core::crypto::Pair;
use std::{collections::HashMap, path::PathBuf, string::String, sync::Arc};

pub(crate) fn init_enclave(
	mu_ra_url: String,
	untrusted_worker_url: String,
	base_dir: PathBuf,
) -> EnclaveResult<()> {
	let signing_key_repository = Arc::new(get_ed25519_repository(base_dir.clone())?);
	GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT.initialize(signing_key_repository.clone());
	let signer = signing_key_repository.retrieve_key()?;
	info!("[Enclave initialized] Ed25519 prim raw : {:?}", signer.public().0);

	let shielding_key_repository = Arc::new(get_rsa3072_repository(base_dir.clone())?);
	GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT.initialize(shielding_key_repository.clone());

	// Create the aes key that is used for state encryption such that a key is always present in tests.
	// It will be overwritten anyway if mutual remote attestation is performed with the primary worker.
	let state_key_repository = Arc::new(get_aes_repository(base_dir.clone())?);
	GLOBAL_STATE_KEY_REPOSITORY_COMPONENT.initialize(state_key_repository.clone());

	let integritee_light_client_seal = Arc::new(EnclaveLightClientSeal::new(
		base_dir.join(LITENTRY_PARENTCHAIN_LIGHT_CLIENT_DB_PATH),
		ParentchainId::Litentry,
	)?);
	GLOBAL_INTEGRITEE_PARENTCHAIN_LIGHT_CLIENT_SEAL.initialize(integritee_light_client_seal);

	let target_a_light_client_seal = Arc::new(EnclaveLightClientSeal::new(
		base_dir.join(TARGET_A_PARENTCHAIN_LIGHT_CLIENT_DB_PATH),
		ParentchainId::TargetA,
	)?);
	GLOBAL_TARGET_A_PARENTCHAIN_LIGHT_CLIENT_SEAL.initialize(target_a_light_client_seal);

	let target_b_light_client_seal = Arc::new(EnclaveLightClientSeal::new(
		base_dir.join(TARGET_B_PARENTCHAIN_LIGHT_CLIENT_DB_PATH),
		ParentchainId::TargetB,
	)?);
	GLOBAL_TARGET_B_PARENTCHAIN_LIGHT_CLIENT_SEAL.initialize(target_b_light_client_seal);

	let state_file_io =
		Arc::new(EnclaveStateFileIo::new(state_key_repository, StateDir::new(base_dir)));
	let state_initializer =
		Arc::new(EnclaveStateInitializer::new(shielding_key_repository.clone()));
	let state_snapshot_repository_loader = StateSnapshotRepositoryLoader::<
		EnclaveStateFileIo,
		EnclaveStateInitializer,
	>::new(state_file_io, state_initializer.clone());

	let state_snapshot_repository =
		state_snapshot_repository_loader.load_snapshot_repository(STATE_SNAPSHOTS_CACHE_SIZE)?;
	let state_observer = initialize_state_observer(&state_snapshot_repository)?;
	GLOBAL_STATE_OBSERVER_COMPONENT.initialize(state_observer.clone());

	let state_handler = Arc::new(StateHandler::load_from_repository(
		state_snapshot_repository,
		state_observer.clone(),
		state_initializer,
	)?);

	GLOBAL_STATE_HANDLER_COMPONENT.initialize(state_handler.clone());

	let ocall_api = Arc::new(OcallApi);
	GLOBAL_OCALL_API_COMPONENT.initialize(ocall_api.clone());

	// For debug purposes, list shards. no problem to panic if fails.
	#[allow(clippy::unwrap_used)]
	let shards = state_handler.list_shards().unwrap();
	debug!("found the following {} shards on disk:", shards.len());
	for s in shards {
		debug!("{}", s.encode().to_base58())
	}

	itp_primitives_cache::set_primitives(
		GLOBAL_PRIMITIVES_CACHE.as_ref(),
		mu_ra_url,
		untrusted_worker_url,
	)
	.map_err(Error::PrimitivesAccess)?;

	let watch_extractor = Arc::new(create_determine_watch::<Hash>());

	let connection_registry = Arc::new(ConnectionRegistry::<Hash, ConnectionToken>::new());

	// We initialize components for the public RPC / direct invocation server here, so we can start the server
	// before registering on the parentchain. If we started the RPC AFTER registering on the parentchain and
	// initializing the light-client, there is a period of time where a peer might want to reach us,
	// but the RPC server is not yet up and running, resulting in error messages or even in that
	// validateer completely breaking (IO PipeError).
	// Corresponding GH issues are #545 and #600.

	let response_channel = Arc::new(RpcResponseChannel::default());
	let rpc_responder =
		Arc::new(EnclaveRpcResponder::new(connection_registry.clone(), response_channel));

	let (request_sink, broadcaster) = init(rpc_responder.clone());
	let request_sink_cloned = request_sink.clone();

	let top_pool_author = create_top_pool_author(
		rpc_responder,
		state_handler.clone(),
		ocall_api.clone(),
		shielding_key_repository.clone(),
		request_sink_cloned,
	);
	GLOBAL_TOP_POOL_AUTHOR_COMPONENT.initialize(top_pool_author.clone());

	GLOBAL_DIRECT_RPC_BROADCASTER_COMPONENT.initialize(broadcaster);
	DIRECT_RPC_REQUEST_SINK_COMPONENT.initialize(request_sink);

	if let Ok(data_provider_config) = DataProviderConfig::new() {
		GLOBAL_DATA_PROVIDER_CONFIG.initialize(data_provider_config.into());
	} else {
		return Err(Error::Other("data provider initialize error".into()))
	}

	let data_provider_config = GLOBAL_DATA_PROVIDER_CONFIG.get()?;
	let getter_executor = Arc::new(EnclaveGetterExecutor::new(state_observer));
	let io_handler = public_api_rpc_handler(
		top_pool_author,
		getter_executor,
		shielding_key_repository,
		Some(state_handler),
		data_provider_config,
	);
	let rpc_handler = Arc::new(RpcWsHandler::new(io_handler, watch_extractor, connection_registry));
	GLOBAL_RPC_WS_HANDLER_COMPONENT.initialize(rpc_handler);

	let sidechain_block_import_queue = Arc::new(EnclaveSidechainBlockImportQueue::default());
	GLOBAL_SIDECHAIN_IMPORT_QUEUE_COMPONENT.initialize(sidechain_block_import_queue);

	let attestation_handler =
		Arc::new(IntelAttestationHandler::new(ocall_api, signing_key_repository));
	GLOBAL_ATTESTATION_HANDLER_COMPONENT.initialize(attestation_handler);

	let evm_assertion_repository = EvmAssertionRepository::new(ASSERTIONS_FILE)?;
	GLOBAL_ASSERTION_REPOSITORY.initialize(evm_assertion_repository.into());

	Ok(())
}

fn initialize_state_observer(
	snapshot_repository: &EnclaveStateSnapshotRepository,
) -> EnclaveResult<Arc<EnclaveStateObserver>> {
	let shards = snapshot_repository.list_shards()?;
	let mut states_map = HashMap::<
		ShardIdentifier,
		<EnclaveStateSnapshotRepository as VersionedStateAccess>::StateType,
	>::new();
	for shard in shards.into_iter() {
		let state = snapshot_repository.load_latest(&shard)?;
		states_map.insert(shard, state);
	}
	Ok(Arc::new(EnclaveStateObserver::from_map(states_map)))
}

fn run_stf_task_handler() -> Result<(), Error> {
	let author_api = GLOBAL_TOP_POOL_AUTHOR_COMPONENT.get()?;
	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;
	let state_observer = GLOBAL_STATE_OBSERVER_COMPONENT.get()?;
	let data_provider_config = GLOBAL_DATA_PROVIDER_CONFIG.get()?;
	let evm_assertion_repository = GLOBAL_ASSERTION_REPOSITORY.get()?;

	let shielding_key_repository = GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT.get()?;

	let ocall_api = GLOBAL_OCALL_API_COMPONENT.get()?;
	let stf_enclave_signer = Arc::new(EnclaveStfEnclaveSigner::new(
		state_observer,
		ocall_api.clone(),
		shielding_key_repository.clone(),
		author_api.clone(),
	));

	let stf_task_context = StfTaskContext::new(
		shielding_key_repository,
		author_api,
		stf_enclave_signer,
		state_handler,
		ocall_api,
		data_provider_config,
		evm_assertion_repository,
	);

	run_stf_task_receiver(Arc::new(stf_task_context)).map_err(Error::StfTaskReceiver)
}

fn run_vc_issuance() -> Result<(), Error> {
	let author_api = GLOBAL_TOP_POOL_AUTHOR_COMPONENT.get()?;
	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;
	let state_observer = GLOBAL_STATE_OBSERVER_COMPONENT.get()?;
	let data_provider_config = GLOBAL_DATA_PROVIDER_CONFIG.get()?;
	let evm_assertion_repository = GLOBAL_ASSERTION_REPOSITORY.get()?;

	let shielding_key_repository = GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT.get()?;
	#[allow(clippy::unwrap_used)]
	let ocall_api = GLOBAL_OCALL_API_COMPONENT.get()?;
	let stf_enclave_signer = Arc::new(EnclaveStfEnclaveSigner::new(
		state_observer,
		ocall_api.clone(),
		shielding_key_repository.clone(),
		author_api.clone(),
	));

	let stf_task_context = StfTaskContext::new(
		shielding_key_repository,
		author_api,
		stf_enclave_signer,
		state_handler,
		ocall_api,
		data_provider_config,
		evm_assertion_repository,
	);
	let extrinsic_factory = get_extrinsic_factory_from_integritee_solo_or_parachain()?;
	let node_metadata_repo = get_node_metadata_repository_from_integritee_solo_or_parachain()?;

	run_vc_handler_runner(Arc::new(stf_task_context), extrinsic_factory, node_metadata_repo);

	Ok(())
}

pub(crate) fn init_enclave_sidechain_components(
	fail_mode: Option<String>,
	fail_at: u64,
) -> EnclaveResult<()> {
	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;
	let ocall_api = GLOBAL_OCALL_API_COMPONENT.get()?;
	let direct_rpc_broadcaster = GLOBAL_DIRECT_RPC_BROADCASTER_COMPONENT.get()?;

	let top_pool_author = GLOBAL_TOP_POOL_AUTHOR_COMPONENT.get()?;
	let state_key_repository = GLOBAL_STATE_KEY_REPOSITORY_COMPONENT.get()?;

	// GLOBAL_SCHEDULED_ENCLAVE must be initialized after attestation_handler and enclave
	let attestation_handler = GLOBAL_ATTESTATION_HANDLER_COMPONENT.get()?;
	let mrenclave = attestation_handler.get_mrenclave()?;
	GLOBAL_SCHEDULED_ENCLAVE.init(mrenclave).map_err(|e| Error::Other(e.into()))?;

	let parentchain_block_import_dispatcher =
		get_triggered_dispatcher_from_integritee_solo_or_parachain()?;

	let signer = GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT.get()?.retrieve_key()?;

	let sidechain_block_importer = Arc::new(EnclaveSidechainBlockImporter::new(
		state_handler,
		state_key_repository.clone(),
		top_pool_author,
		parentchain_block_import_dispatcher,
		ocall_api.clone(),
		direct_rpc_broadcaster,
	));

	let sidechain_block_import_queue = GLOBAL_SIDECHAIN_IMPORT_QUEUE_COMPONENT.get()?;
	let metadata_repository = get_node_metadata_repository_from_integritee_solo_or_parachain()?;
	let extrinsics_factory = get_extrinsic_factory_from_integritee_solo_or_parachain()?;
	let validator_accessor = get_validator_accessor_from_integritee_solo_or_parachain()?;

	let sidechain_block_import_confirmation_handler =
		Arc::new(EnclaveBlockImportConfirmationHandler::new(
			metadata_repository,
			extrinsics_factory,
			validator_accessor,
		));

	let sidechain_block_syncer = Arc::new(EnclaveSidechainBlockSyncer::new(
		sidechain_block_importer,
		ocall_api,
		sidechain_block_import_confirmation_handler,
	));
	GLOBAL_SIDECHAIN_BLOCK_SYNCER_COMPONENT.initialize(sidechain_block_syncer.clone());

	let sidechain_block_import_queue_worker =
		Arc::new(EnclaveSidechainBlockImportQueueWorker::new(
			sidechain_block_import_queue,
			sidechain_block_syncer,
		));
	GLOBAL_SIDECHAIN_IMPORT_QUEUE_WORKER_COMPONENT.initialize(sidechain_block_import_queue_worker);

	let block_composer = Arc::new(BlockComposer::new(signer, state_key_repository));
	GLOBAL_SIDECHAIN_BLOCK_COMPOSER_COMPONENT.initialize(block_composer);
	if let Some(fail_mode) = fail_mode {
		let fail_mode = FailSlotMode::from_str(&fail_mode)
			.map_err(|_| Error::Sgx(sgx_status_t::SGX_ERROR_UNEXPECTED))?;
		let fail_on_demand = Arc::new(Some(FailSlotOnDemand::new(fail_at, fail_mode)));
		GLOBAL_SIDECHAIN_FAIL_SLOT_ON_DEMAND_COMPONENT.initialize(fail_on_demand);
	} else {
		GLOBAL_SIDECHAIN_FAIL_SLOT_ON_DEMAND_COMPONENT.initialize(Arc::new(None));
	}

	std::thread::spawn(move || {
		println!("running stf task handler");
		#[allow(clippy::unwrap_used)]
		run_stf_task_handler().unwrap();
	});

	std::thread::spawn(move || {
		println!("running vc issuance");
		#[allow(clippy::unwrap_used)]
		run_vc_issuance().unwrap();
	});

	Ok(())
}

pub(crate) fn init_direct_invocation_server(server_addr: String) -> EnclaveResult<()> {
	info!("Initialize direct invocation server on {}", &server_addr);
	let rpc_handler = GLOBAL_RPC_WS_HANDLER_COMPONENT.get()?;
	let signer = GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT.get()?.retrieve_key()?;

	let url = url::Url::parse(&server_addr).map_err(|e| Error::Other(format!("{}", e).into()))?;
	let maybe_config_provider = if url.scheme() == "wss" {
		let cert = ed25519_self_signed_certificate(signer, "Enclave")
			.map_err(|e| Error::Other(e.into()))?;

		// Serialize certificate(s) and private key to PEM.
		// PEM format is needed as a certificate chain can only be serialized into PEM.
		let pem_serialized = cert.serialize_pem().map_err(|e| Error::Other(e.into()))?;
		let private_key = cert.serialize_private_key_pem();

		Some(Arc::new(FromFileConfigProvider::new(private_key, pem_serialized)))
	} else {
		None
	};

	let web_socket_server = Arc::new(TungsteniteWsServer::new(
		url.authority().into(),
		maybe_config_provider,
		rpc_handler,
	));

	GLOBAL_WEB_SOCKET_SERVER_COMPONENT.initialize(web_socket_server.clone());

	match web_socket_server.run() {
		Ok(()) => {},
		Err(e) => {
			error!("Web socket server encountered an unexpected error: {:?}", e)
		},
	}

	Ok(())
}

pub(crate) fn init_shard(shard: ShardIdentifier) -> EnclaveResult<()> {
	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;
	let _ = state_handler.initialize_shard(shard)?;
	Ok(())
}

pub(crate) fn migrate_shard(
	old_shard: ShardIdentifier,
	new_shard: ShardIdentifier,
) -> EnclaveResult<()> {
	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;
	let _ = state_handler.migrate_shard(old_shard, new_shard)?;
	Ok(())
}

/// Initialize the TOP pool author component.
pub fn create_top_pool_author(
	rpc_responder: Arc<EnclaveRpcResponder>,
	state_handler: Arc<EnclaveStateHandler>,
	ocall_api: Arc<EnclaveOCallApi>,
	shielding_key_repository: Arc<EnclaveShieldingKeyRepository>,
	requests_sink: Arc<std::sync::mpsc::SyncSender<BroadcastedRequest>>,
) -> Arc<EnclaveTopPoolAuthor> {
	let side_chain_api = Arc::new(EnclaveSidechainApi::new());
	let top_pool =
		Arc::new(EnclaveTopPool::create(PoolOptions::default(), side_chain_api, rpc_responder));

	Arc::new(EnclaveTopPoolAuthor::new(
		top_pool,
		AuthorTopFilter::<TrustedCallSigned, Getter>::new(),
		BroadcastedTopFilter::<TrustedCallSigned, Getter>::new(),
		state_handler,
		shielding_key_repository,
		ocall_api,
		requests_sink,
	))
}
