use cw_orch::{interface, prelude::*};
use streamswap_types::stream::{ExecuteMsg, QueryMsg, MigrateMsg};
use streamswap_types::factory::CreateStreamMsg as InstantiateMsg;
use streamswap_factory::contract::{execute, instantiate, migrate, query};

pub const CONTRACT_ID: &str = "streamswap_stream";

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg, id = CONTRACT_ID)]
pub struct StreamSwapStreamContract;

impl<Chain> Uploadable for StreamSwapStreamContract<Chain> {
    /// Return the path to the wasm file corresponding to the contract
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path(CONTRACT_ID)
            .unwrap()
    }
    /// Returns a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                execute,
                instantiate,
                query,
            )
                .with_migrate(migrate)
        )
    }
}