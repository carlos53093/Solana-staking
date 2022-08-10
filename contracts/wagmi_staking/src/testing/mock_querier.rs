use crate::querier::{KongzExtension, KongzMetaData, MonkeezNftResponse, MonkeezQueryMsg};
use cosmwasm_std::{
    from_binary, from_slice,
    testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    to_binary, Addr, Coin, ContractResult, OwnedDeps, Querier, QuerierResult, QueryRequest,
    SystemError, SystemResult, WasmQuery,
};
use cw721::{Cw721QueryMsg, NftInfoResponse, TokensResponse};
use schemars::_serde_json::to_string;
use std::str::FromStr;
use terra_cosmwasm::TerraQueryWrapper;

pub fn mock_dependencies_custom(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, WasmMockQuerier> {
    let custom_querier: WasmMockQuerier =
        WasmMockQuerier::new(MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)]));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}

pub struct WasmMockQuerier {
    base: MockQuerier<TerraQueryWrapper>,
}

impl Querier for WasmMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        // MockQuerier doesn't support Custom, so we ignore it completely here
        let request: QueryRequest<TerraQueryWrapper> = match from_slice(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {}", e),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl WasmMockQuerier {
    pub fn handle_query(&self, request: &QueryRequest<TerraQueryWrapper>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                if contract_addr == &Addr::unchecked("KONGZ_NFT") {
                    match from_binary(msg).unwrap() {
                        Cw721QueryMsg::NftInfo { token_id } => {
                            let k = u64::from_str(&token_id).unwrap();
                            let msg_response = NftInfoResponse {
                                token_uri: None,
                                extension: KongzExtension {
                                    image: format!("KONGZ_IMG{}", k),
                                    image_data: None,
                                    external_url: None,
                                    description: "KONG_DES1".to_string(),
                                    name: format!("#{}", k),
                                    attributes: vec![KongzMetaData {
                                        display_type: None,
                                        trait_type: "Rarity".to_string(),
                                        value: "Legendary".to_string(),
                                    }],
                                    background_color: None,
                                    animation_url: None,
                                    youtube_url: None,
                                },
                            };
                            return SystemResult::Ok(ContractResult::Ok(
                                to_binary(&msg_response).unwrap(),
                            ));
                        }
                        _ => self.base.handle_query(request),
                    }
                } else if contract_addr == &Addr::unchecked("MONKEEZ_NFT") {
                    match from_binary(msg).unwrap() {
                        MonkeezQueryMsg::NftAdditionalInfo { token_id } => {
                            let k = u64::from_str(&token_id).unwrap();
                            let msg_response =
                                MonkeezNftResponse {
                                    owner: "creator".to_string(),
                                    token_id,
                                    name: format!("#{}",k),
                                    description: format!("MONKEEZ_DES{}",k),
                                    image: None,
                                    uri: None,
                                    metadata: "[{\"name\":\"background\",\"value\":\"dracula\",\"rarity_name\":\"legendary\"}]".to_string(),
                                    creator: "creator_0".to_string(),
                                    royalty_percent_fee: None
                                };
                            return SystemResult::Ok(ContractResult::Ok(
                                to_binary(&msg_response).unwrap(),
                            ));
                        }
                        _ => self.base.handle_query(request),
                    }
                } else if contract_addr == &Addr::unchecked("REWARD_NFT") {
                    match from_binary(msg).unwrap() {
                        Cw721QueryMsg::Tokens {
                            owner: _,
                            start_after: start_after,
                            limit: limit,
                        } => {
                            if let Some(start_after) = start_after {
                                match start_after.as_str() {
                                    "30" => {
                                        let msg_response = TokensResponse {
                                            tokens: vec![
                                                "31".to_string(),
                                                "32".to_string(),
                                                "33".to_string(),
                                                "34".to_string(),
                                                "35".to_string(),
                                                "36".to_string(),
                                                "37".to_string(),
                                                "38".to_string(),
                                                "39".to_string(),
                                                "40".to_string(),
                                                "41".to_string(),
                                                "42".to_string(),
                                                "43".to_string(),
                                                "44".to_string(),
                                                "45".to_string(),
                                                "46".to_string(),
                                                "47".to_string(),
                                                "48".to_string(),
                                                "49".to_string(),
                                                "50".to_string(),
                                                "51".to_string(),
                                                "52".to_string(),
                                                "53".to_string(),
                                                "54".to_string(),
                                                "55".to_string(),
                                                "56".to_string(),
                                                "57".to_string(),
                                                "58".to_string(),
                                                "59".to_string(),
                                                "60".to_string(),
                                            ],
                                        };
                                        return SystemResult::Ok(ContractResult::Ok(
                                            to_binary(&msg_response).unwrap(),
                                        ));
                                    }
                                    "60" => {
                                        let msg_response = TokensResponse {
                                            tokens: vec![
                                                "61".to_string(),
                                                "62".to_string(),
                                                "63".to_string(),
                                                "64".to_string(),
                                                "65".to_string(),
                                                "66".to_string(),
                                                "67".to_string(),
                                                "68".to_string(),
                                                "69".to_string(),
                                                "70".to_string(),
                                                "71".to_string(),
                                                "72".to_string(),
                                                "73".to_string(),
                                                "74".to_string(),
                                                "75".to_string(),
                                                "76".to_string(),
                                                "77".to_string(),
                                                "78".to_string(),
                                                "79".to_string(),
                                                "80".to_string(),
                                                "81".to_string(),
                                                "82".to_string(),
                                                "83".to_string(),
                                                "84".to_string(),
                                                "85".to_string(),
                                                "86".to_string(),
                                                "87".to_string(),
                                                "88".to_string(),
                                                "89".to_string(),
                                                "90".to_string(),
                                            ],
                                        };
                                        return SystemResult::Ok(ContractResult::Ok(
                                            to_binary(&msg_response).unwrap(),
                                        ));
                                    }
                                    _ => {
                                        let msg_response = TokensResponse { tokens: vec![] };
                                        return SystemResult::Ok(ContractResult::Ok(
                                            to_binary(&msg_response).unwrap(),
                                        ));
                                    }
                                }
                            } else {
                                let msg_response = TokensResponse {
                                    tokens: vec![
                                        "1".to_string(),
                                        "2".to_string(),
                                        "3".to_string(),
                                        "4".to_string(),
                                        "5".to_string(),
                                        "6".to_string(),
                                        "7".to_string(),
                                        "8".to_string(),
                                        "9".to_string(),
                                        "10".to_string(),
                                        "11".to_string(),
                                        "12".to_string(),
                                        "13".to_string(),
                                        "14".to_string(),
                                        "15".to_string(),
                                        "16".to_string(),
                                        "17".to_string(),
                                        "18".to_string(),
                                        "19".to_string(),
                                        "20".to_string(),
                                        "21".to_string(),
                                        "22".to_string(),
                                        "23".to_string(),
                                        "24".to_string(),
                                        "25".to_string(),
                                        "26".to_string(),
                                        "27".to_string(),
                                        "28".to_string(),
                                        "29".to_string(),
                                        "30".to_string(),
                                    ],
                                };
                                return SystemResult::Ok(ContractResult::Ok(
                                    to_binary(&msg_response).unwrap(),
                                ));
                            }
                        }
                        _ => self.base.handle_query(request),
                    }
                } else {
                    self.base.handle_query(request)
                }
            }
            _ => self.base.handle_query(request),
        }
    }

    pub fn new(base: MockQuerier<TerraQueryWrapper>) -> Self {
        WasmMockQuerier { base }
    }
}
