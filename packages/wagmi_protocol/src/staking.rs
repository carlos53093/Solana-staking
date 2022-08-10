use cw721::Cw721ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub monkeez_nft: String,
    pub kongz_nft: String,
    pub reward_nft: String, //NFT token contract
    pub tokens_owner: String, // reward token's owner,
                            // pub legendaries_ids: Vec<String>, //  legendaries_token_ids for first 3 reward
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ReceiveNft(Cw721ReceiveMsg),
    Unstake {
        token_kind: u64, // 0: monkeez, 1: kongz
        token_id: String,
    },
    ClaimReward {},
    Update {
        owner: Option<String>,
        monkeez_nft: Option<String>,
        kongz_nft: Option<String>,
        reward_token: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Cw721HookMsg {
    Stake {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    // Reward { staker: String },
    StakedTokens { owner: String },
    Reward { staker: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ConfigResponse {
    pub owner: String,
    pub monkeez_nft: String,
    pub kongz_nft: String,
    pub reward_nft: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ClusterReward {
    pub claimable_num: u64,
    pub remain_time: Option<u64>, // seconds
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct RewardResponse {
    pub claimable_amount: u64,
    pub cluster_rewards: Vec<ClusterReward>, //claimable_num, remain_time
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub token_kind: u64,
    pub token_id: String,
    pub is_common: bool,
}

impl TokenInfo {
    pub fn is_match(&self, token_kind: u64, token_id: &String) -> bool {
        self.token_kind == token_kind && self.token_id.as_str() == token_id.as_str()
    }
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ClusterResponse {
    pub tokens: Vec<TokenInfo>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct TokensInfoResponse {
    pub clusters: Vec<ClusterResponse>,
}
