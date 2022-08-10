use cosmwasm_std::{to_binary, CanonicalAddr, Deps, QueryRequest, StdResult, WasmQuery};
use cw721::{Cw721QueryMsg, NftInfoResponse, TokensResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MonkeezQueryMsg {
    NftAdditionalInfo { token_id: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MonkeezMetaData {
    pub name: String,
    pub value: String,
    pub rarity_name: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct KongzMetaData {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MonkeezNftResponse {
    pub owner: String,
    pub token_id: String,
    pub name: String,
    pub description: String,
    pub image: Option<String>,
    pub uri: Option<String>,
    pub metadata: String,
    pub creator: String,
    pub royalty_percent_fee: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct KongzExtension {
    pub image: String,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: String,
    pub name: String,
    pub attributes: Vec<KongzMetaData>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

// pub fn query_token_owner(
//     deps: Deps,
//     contract_addr: &CanonicalAddr,
//     token_id: &String,
// ) -> StdResult<String> {
//     let owner: OwnerOfResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
//         contract_addr: deps.api.addr_humanize(contract_addr).unwrap().to_string(),
//         msg: to_binary(&Cw721QueryMsg::OwnerOf {
//             token_id: token_id.to_string(),
//             include_expired: None,
//         })?,
//     }))?;
//     Ok(owner.owner)
// }

// pub fn query_tokenid_from_owner(
//     deps: Deps,
//     nft_addr: String,
//     owner: String,
// ) -> StdResult<Vec<String>> {
//     let tokens: TokensResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
//         contract_addr: nft_addr,
//         msg: to_binary(&Cw721QueryMsg::Tokens {
//             owner,
//             start_after: None,
//             limit: Some(30),
//         })?,
//     }))?;
//     Ok(tokens.tokens)
// }
pub fn query_all_tokenid_from_owner(
    deps: Deps,
    nft_addr: String,
    owner: String,
) -> StdResult<Vec<String>> {
    let mut token_ids: Vec<String> = vec![];
    let mut start_after: Option<String> = None;
    loop {
        let tokens: TokensResponse = deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: nft_addr.clone(),
            msg: to_binary(&Cw721QueryMsg::Tokens {
                owner: owner.clone(),
                start_after: start_after.clone(),
                limit: Some(30),
            })?,
        }))?;
        for x in tokens.tokens.iter() {
            token_ids.push(x.clone());
        }

        if tokens.tokens.len() < 30 {
            break;
        }
        start_after = Some(tokens.tokens[29].clone());
    }
    Ok(token_ids)
}

pub fn is_common_kongz(
    deps: Deps,
    contract_addr: &CanonicalAddr,
    token_id: &String,
) -> StdResult<bool> {
    let nft_info: NftInfoResponse<KongzExtension> =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.addr_humanize(contract_addr).unwrap().to_string(),
            msg: to_binary(&Cw721QueryMsg::NftInfo {
                token_id: token_id.to_string(),
            })?,
        }))?;
    let metadata = nft_info
        .extension
        .attributes
        .iter()
        .find(|&x| x.trait_type == "Rarity" && x.value == "Common");

    Ok(metadata.is_some())
}

pub fn is_common_monkeez(
    deps: Deps,
    contract_addr: &CanonicalAddr,
    token_id: &String,
) -> StdResult<bool> {
    let nft_info: MonkeezNftResponse =
        deps.querier.query(&QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: deps.api.addr_humanize(contract_addr).unwrap().to_string(),
            msg: to_binary(&MonkeezQueryMsg::NftAdditionalInfo {
                token_id: token_id.to_string(),
            })?,
        }))?;
    let metadata = nft_info.metadata;
    Ok(!metadata.contains("\"rarity_name\":\"legendary\""))
}
