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
use crate::{error::Result, NodeMetadata};

/// Pallet' name:
pub const TEEBAG: &str = "Teebag";

// we only list the extrinsics that we care
pub trait TeebagCallIndexes {
	fn set_scheduled_enclave_call_indexes(&self) -> Result<[u8; 2]>;

	fn remove_scheduled_enclave_call_indexes(&self) -> Result<[u8; 2]>;

	fn register_enclave_call_indexes(&self) -> Result<[u8; 2]>;

	fn unregister_enclave_call_indexes(&self) -> Result<[u8; 2]>;

	fn register_quoting_enclave_call_indexes(&self) -> Result<[u8; 2]>;

	fn register_tcb_info_call_indexes(&self) -> Result<[u8; 2]>;

	fn post_opaque_task_call_indexes(&self) -> Result<[u8; 2]>;

	fn parentchain_block_processed_call_indexes(&self) -> Result<[u8; 2]>;

	fn sidechain_block_imported_call_indexes(&self) -> Result<[u8; 2]>;
}

impl TeebagCallIndexes for NodeMetadata {
	fn set_scheduled_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "set_scheduled_enclave")
	}
	fn remove_scheduled_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "remove_scheduled_enclave")
	}
	fn register_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "register_enclave")
	}
	fn unregister_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "unregister_enclave")
	}
	fn register_quoting_enclave_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "register_quoting_enclave")
	}
	fn register_tcb_info_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "register_tcb_info")
	}
	fn post_opaque_task_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "post_opaque_task")
	}
	fn parentchain_block_processed_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "parentchain_block_processed")
	}
	fn sidechain_block_imported_call_indexes(&self) -> Result<[u8; 2]> {
		self.call_indexes(TEEBAG, "sidechain_block_imported")
	}
}
