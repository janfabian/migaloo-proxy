use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::vault;
use terraswap::asset::AssetInfo;
use white_whale::fee::VaultFee;

/// The instantiation message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The owner of the factory
    pub owner: String,
    /// The code ID for the vault contract
    pub vault_id: u64,
    /// The code ID for the liquidity token contract
    pub token_id: u64,
    /// The address where fees get collected
    pub fee_collector_addr: String,
}

/// The execution message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Creates a new vault given the asset info the vault should manage deposits and withdrawals
    /// for and the fees
    CreateVault {
        asset_info: AssetInfo,
        fees: VaultFee,
    },
    /// Migrates vaults to the given code_id. If a [vault_addr] is provided, then migrates only that
    /// vault.
    MigrateVaults {
        vault_addr: Option<String>,
        vault_code_id: u64,
    },
    /// Updates a vault config
    UpdateVaultConfig {
        vault_addr: String,
        params: vault::UpdateConfigParams,
    },
    /// Updates the configuration of the vault factory.
    /// If a field is not specified, it will not be modified.
    UpdateConfig {
        owner: Option<String>,
        fee_collector_addr: Option<String>,
        vault_id: Option<u64>,
        token_id: Option<u64>,
    },
}

/// The query message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Retrieves the configuration of the vault. Returns a [`Config`] struct.
    Config {},
    /// Retrieves the address of a given vault. Returns an [`Option<String>`].
    Vault { asset_info: AssetInfo },
    /// Retrieves the addresses for all the vaults. Returns an [`Option<Vec<String>>`].
    Vaults {
        start_after: Option<Vec<u8>>,
        limit: Option<u32>,
    },
}

/// The migrate message
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

/// The `reply` code ID for the submessage after instantiating the vault.
pub const INSTANTIATE_VAULT_REPLY_ID: u64 = 1;

/// Response for the vaults query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VaultsResponse {
    pub vaults: Vec<VaultInfo>,
}

/// Response for the vaults query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct VaultInfo {
    pub vault: String,
    pub asset_info_reference: Vec<u8>,
}
