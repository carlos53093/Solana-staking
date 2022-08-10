use cosmwasm_std::{CanonicalAddr, Decimal, StdResult, Storage};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use wagmi_protocol::staking::TokenInfo;

pub const CONFIG: Item<Config> = Item::new("config");
pub const HOLDERS: Map<&[u8], Holder> = Map::new("holders");
// pub const NEW_TOKEN_ID: Item<u64> = Item::new("new_token_id");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: CanonicalAddr,
    pub monkeez_nft: CanonicalAddr,
    pub kongz_nft: CanonicalAddr,
    pub reward_nft: CanonicalAddr,
    pub tokens_owner: CanonicalAddr,
    // pub legendaries_ids: Vec<String>,
}
impl Config {
    pub fn staked_nft_addr(&self, selector: u64) -> Option<&CanonicalAddr> {
        match selector {
            0 => Some(&self.monkeez_nft),
            1 => Some(&self.kongz_nft),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Cluster {
    pub token_ids: Vec<TokenInfo>,
    pub last_reward_time: u64,
    pub last_reward_earned: Decimal,
    pub last_reward_release: Decimal,
}
impl Cluster {
    pub fn count_legendary(&self) -> u64 {
        self.token_ids.iter().filter(|&x| !x.is_common).count() as u64
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Holder {
    // pub token_ids: Vec<TokenInfo>,
    pub clusters: Vec<Cluster>,
}

// impl Holder {
//     pub fn cluster(&self) -> Vec<Cluster> {
//         let mut clusters: Vec<Cluster> = vec![];
//         let mut i: u64 = 0;
//         let mut j: usize = 0;
//         for token_id in self.token_ids.iter() {
//             if i % 5 == 0 {
//                 clusters.push(Cluster { token_ids: vec![] });
//                 j += 1;
//             }
//             clusters[j - 1].token_ids.push(token_id.clone());
//             i += 1;
//         }
//         clusters
//     }
// }

pub fn store_holder(
    storage: &mut dyn Storage,
    holder_address: &CanonicalAddr,
    holder: &Holder,
) -> StdResult<()> {
    HOLDERS.save(storage, holder_address.as_slice(), holder)
}

pub fn read_holder(storage: &dyn Storage, holder_address: &CanonicalAddr) -> StdResult<Holder> {
    let res = HOLDERS.may_load(storage, holder_address.as_slice())?;
    match res {
        Some(holder) => Ok(holder),
        None => Ok(Holder { clusters: vec![] }),
    }
}
