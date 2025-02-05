use crate::{error::Error, filter_metadata::EventsFromMetadata};
use itp_stf_primitives::traits::IndirectExecutor;
use itp_test::mock::stf_mock::TrustedCallSignedMock;
use itp_types::{
	parentchain::{
		events::{
			ActivateIdentityRequested, AssertionCreated, DeactivateIdentityRequested,
			LinkIdentityRequested, OpaqueTaskPosted, ScheduledEnclaveRemoved, ScheduledEnclaveSet,
			VCRequested,
		},
		FilterEvents, HandleParentchainEvents,
	},
	RsaRequest, H256,
};
use std::vec::Vec;

pub struct TestEventCreator;

impl<NodeMetadata> EventsFromMetadata<NodeMetadata> for TestEventCreator {
	type Output = MockEvents;

	fn create_from_metadata(
		_metadata: NodeMetadata,
		_block_hash: H256,
		_events: &[u8],
	) -> Option<Self::Output> {
		Some(MockEvents)
	}
}

pub struct MockEvents;

impl FilterEvents for MockEvents {
	type Error = ();

	fn get_opaque_task_posted_events(&self) -> Result<Vec<OpaqueTaskPosted>, Self::Error> {
		let opaque_task_posted_event =
			OpaqueTaskPosted { request: RsaRequest::new(H256::default(), Vec::from([0u8; 32])) };
		Ok(Vec::from([opaque_task_posted_event]))
	}

	fn get_link_identity_events(&self) -> Result<Vec<LinkIdentityRequested>, Self::Error> {
		Ok(Vec::new())
	}

	fn get_vc_requested_events(&self) -> Result<Vec<VCRequested>, Self::Error> {
		Ok(Vec::new())
	}

	fn get_deactivate_identity_events(
		&self,
	) -> Result<Vec<DeactivateIdentityRequested>, Self::Error> {
		Ok(Vec::new())
	}

	fn get_activate_identity_events(&self) -> Result<Vec<ActivateIdentityRequested>, Self::Error> {
		Ok(Vec::new())
	}

	fn get_scheduled_enclave_set_events(&self) -> Result<Vec<ScheduledEnclaveSet>, Self::Error> {
		Ok(Vec::new())
	}

	fn get_scheduled_enclave_removed_events(
		&self,
	) -> Result<Vec<ScheduledEnclaveRemoved>, Self::Error> {
		Ok(Vec::new())
	}

	fn get_assertion_created_events(&self) -> Result<Vec<AssertionCreated>, Self::Error> {
		Ok(Vec::new())
	}
}

pub struct MockParentchainEventHandler {}

impl<Executor> HandleParentchainEvents<Executor, TrustedCallSignedMock, Error>
	for MockParentchainEventHandler
where
	Executor: IndirectExecutor<TrustedCallSignedMock, Error>,
{
	fn handle_events(&self, _: &Executor, _: impl FilterEvents) -> Result<Vec<H256>, Error> {
		Ok(Vec::from([H256::default()]))
	}
}
