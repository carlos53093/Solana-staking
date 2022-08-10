use crate::contract::{
    execute_claim_reward, execute_stake, execute_unstake, instantiate, query_cluster_reward,
    query_staked_tokens,
};
use crate::testing::mock_querier::mock_dependencies_custom;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{attr, to_binary, CosmosMsg, Response, WasmMsg};
use cw721::Cw721ExecuteMsg;
use wagmi_protocol::staking::{
    ClusterResponse, InstantiateMsg, RewardResponse, TokenInfo, TokensInfoResponse,
};

#[test]
fn test_general() {
    let mut deps = mock_dependencies_custom(&[]);
    let env = mock_env();
    let info = mock_info("creator", &[]);
    let init_msg = InstantiateMsg {
        monkeez_nft: "MONKEEZ_NFT".to_string(),
        kongz_nft: "KONGZ_NFT".to_string(),
        reward_nft: "REWARD_NFT".to_string(),
        tokens_owner: "REWARD_TOKENS_OWNER".to_string(),
        // legendaries_ids: vec!["1".to_string(), "2".to_string(), "3".to_string()],
    };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    let expected_res = Response::new().add_attributes(vec![
        attr("action", "instantiate"),
        attr("owner", "creator"),
        attr("monkeez_nft", "MONKEEZ_NFT"),
        attr("kongz_nft", "KONGZ_NFT"),
        attr("reward_nft", "REWARD_NFT"),
    ]);
    assert_eq!(res, expected_res);
    //stake  KONGZ legendary
    let _res = execute_stake(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "creator".to_string(),
        "1".to_string(),
        1,
    )
    .unwrap();
    let _res = execute_stake(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "creator".to_string(),
        "2".to_string(),
        1,
    )
    .unwrap();
    let _res = execute_stake(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "creator".to_string(),
        "3".to_string(),
        1,
    )
    .unwrap();
    let _res = execute_stake(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "creator".to_string(),
        "4".to_string(),
        1,
    )
    .unwrap();
    let _res = execute_stake(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "creator".to_string(),
        "5".to_string(),
        1,
    )
    .unwrap();
    // let _res = execute_stake(
    //     deps.as_mut(),
    //     env.clone(),
    //     info.clone(),
    //     "creator".to_string(),
    //     "6".to_string(),
    //     0,
    // )
    // .unwrap();

    // let _res = execute_stake(
    //     deps.as_mut(),
    //     env.clone(),
    //     info.clone(),
    //     "creator".to_string(),
    //     "7".to_string(),
    //     0,
    // )
    // .unwrap();

    // let expected_res = Response::new().add_attributes(vec![
    //     attr("action", "stake"),
    //     attr("token_kind", "0".to_string()),
    //     attr("token_id", "1".to_string()),
    // ]);
    // assert_eq!(res, expected_res);
    // let res = execute_stake(deps.as_mut(), env.clone(), info.clone(), "creator".to_string(), "2".to_string(), 0).unwrap();
    // let expected_res = Response::new().add_attributes(vec![
    //     attr("action", "stake"),
    //     attr("token_kind", "0".to_string()),
    //     attr("token_id", "2".to_string()),
    // ]);
    // assert_eq!(res, expected_res);

    // let mut env = env.clone();
    // env.block.time = env.block.time.plus_seconds(86400*10); //5 days passed

    // let res = execute_claim_reward(deps.as_mut(), env.clone(), info.clone()).unwrap();
    // let expected_res = Response::new()
    //     .add_messages(vec![
    //         CosmosMsg::Wasm(WasmMsg::Execute {
    //             contract_addr: "REWARD_NFT".to_string(),
    //             msg: to_binary(&Cw721ExecuteMsg::TransferNft {
    //                 recipient: "creator".to_string(),
    //                 token_id: "4".to_string(),
    //             })
    //             .unwrap(),
    //             funds: vec![],
    //         }),
    //         // CosmosMsg::Wasm(WasmMsg::Execute {
    //         //     contract_addr: "REWARD_NFT".to_string(),
    //         //     msg: to_binary(&Cw721ExecuteMsg::TransferNft {
    //         //         recipient: "creator".to_string(),
    //         //         token_id: "2".to_string(),
    //         //     })
    //         //     .unwrap(),
    //         //     funds: vec![],
    //         // }),
    //         // CosmosMsg::Wasm(WasmMsg::Execute {
    //         //     contract_addr: "REWARD_NFT".to_string(),
    //         //     msg: to_binary(&Cw721ExecuteMsg::TransferNft {
    //         //         recipient: "creator".to_string(),
    //         //         token_id: "3".to_string(),
    //         //     })
    //         //     .unwrap(),
    //         //     funds: vec![],
    //         // }),
    //         // CosmosMsg::Wasm(WasmMsg::Execute {
    //         //     contract_addr: "REWARD_NFT".to_string(),
    //         //     msg: to_binary(&Cw721ExecuteMsg::TransferNft {
    //         //         recipient: "creator".to_string(),
    //         //         token_id: "4".to_string(),
    //         //     })
    //         //     .unwrap(),
    //         //     funds: vec![],
    //         // }),
    //     ])
    //     .add_attributes(vec![
    //         attr("action", "claim_reward"),
    //         attr("reward_num", "1"),
    //     ]);
    // assert_eq!(res, expected_res);
    //
    // let res = query_reward(deps.as_ref(), env.clone(), "creator".to_string()).unwrap();
    // let expected_res = RewardResponse{
    //     reward_amount: Uint128::from(0u128)
    // };
    // assert_eq!(res, expected_res);
    //
    //unstake

    // let res =
    //     execute_unstake(deps.as_mut(), env.clone(), info.clone(), 0, "6".to_string()).unwrap();
    //
    // let _res = execute_stake(
    //     deps.as_mut(),
    //     env.clone(),
    //     info.clone(),
    //     "creator".to_string(),
    //     "6".to_string(),
    //     1,
    // )
    //     .unwrap();
    // let expected_res = Response::new()
    //     .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
    //         contract_addr: "MONKEEZ_NFT".to_string(),
    //         msg: to_binary(&Cw721ExecuteMsg::TransferNft {
    //             recipient: "creator".to_string(),
    //             token_id: "1".to_string(),
    //         })
    //         .unwrap(),
    //         funds: vec![],
    //     })])
    //     .add_attributes(vec![
    //         attr("action", "unstake"),
    //         attr("receiver", "creator"),
    //         attr("token_id", "1"),
    //         attr("token_kind", "0"),
    //     ]);
    // assert_eq!(res, expected_res);
    //
    // let mut env = env.clone();
    // env.block.time = env.block.time.plus_seconds(435);
    // let res = query_cluster_reward(deps.as_ref(), env.clone(), "creator".to_string()).unwrap();
    // let expected_res = RewardResponse {
    //     claimable_amount: 1,
    //     cluster_rewards: vec![(
    //         1u64,
    //         Some(791424),
    //         ),
    //         (
    //       0u64,
    //       Some(6307200))
    //     ]
    // };
    // assert_eq!(res, expected_res);

    let res = query_staked_tokens(deps.as_ref(), env.clone(), "creator".to_string()).unwrap();
    let expected_res = TokensInfoResponse {
        clusters: vec![ClusterResponse {
            tokens: vec![
                TokenInfo {
                    token_kind: 1,
                    token_id: "1".to_string(),
                    is_common: false,
                },
                TokenInfo {
                    token_kind: 1,
                    token_id: "2".to_string(),
                    is_common: false,
                },
                TokenInfo {
                    token_kind: 1,
                    token_id: "3".to_string(),
                    is_common: false,
                },
                TokenInfo {
                    token_kind: 1,
                    token_id: "4".to_string(),
                    is_common: false,
                },
                TokenInfo {
                    token_kind: 1,
                    token_id: "5".to_string(),
                    is_common: false,
                },
            ],
        }],
    };
    assert_eq!(res, expected_res);
}

#[test]
fn test_claim_reward() {
    let mut deps = mock_dependencies_custom(&[]);
    let env = mock_env();
    let info = mock_info("creator", &[]);
    let init_msg = InstantiateMsg {
        monkeez_nft: "MONKEEZ_NFT".to_string(),
        kongz_nft: "KONGZ_NFT".to_string(),
        reward_nft: "REWARD_NFT".to_string(),
        tokens_owner: "REWARD_TOKENS_OWNER".to_string(),
        // legendaries_ids: vec!["1".to_string(), "2".to_string(), "3".to_string()],
    };
    let res = instantiate(deps.as_mut(), env.clone(), info.clone(), init_msg).unwrap();
    let expected_res = Response::new().add_attributes(vec![
        attr("action", "instantiate"),
        attr("owner", "creator"),
        attr("monkeez_nft", "MONKEEZ_NFT"),
        attr("kongz_nft", "KONGZ_NFT"),
        attr("reward_nft", "REWARD_NFT"),
    ]);
    assert_eq!(res, expected_res);
    //stake
    let _res = execute_stake(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "creator".to_string(),
        "1".to_string(),
        1,
    )
    .unwrap();
    let _res = execute_stake(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "creator".to_string(),
        "2".to_string(),
        1,
    )
    .unwrap();
    let _res = execute_stake(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "creator".to_string(),
        "3".to_string(),
        1,
    )
    .unwrap();
    let _res = execute_stake(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "creator".to_string(),
        "4".to_string(),
        1,
    )
    .unwrap();
    let _res = execute_stake(
        deps.as_mut(),
        env.clone(),
        info.clone(),
        "creator".to_string(),
        "5".to_string(),
        1,
    )
    .unwrap();
    // let _res = execute_stake(
    //     deps.as_mut(),
    //     env.clone(),
    //     info.clone(),
    //     "creator".to_string(),
    //     "6".to_string(),
    //     0,
    // )
    //     .unwrap();

    let mut env = env.clone();
    env.block.time = env.block.time.plus_seconds(86400 * 16); //16 days passed   16 passed
                                                              //168
                                                              // mock_env blocktime 1572834219
    let res = execute_claim_reward(deps.as_mut(), env.clone(), info.clone()).unwrap();
    let expected_res = Response::new()
        .add_messages(vec![
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: "REWARD_NFT".to_string(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: "creator".to_string(),
                    token_id: "70".to_string(),
                })
                .unwrap(),
                funds: vec![],
            }),
            // CosmosMsg::Wasm(WasmMsg::Execute {
            //     contract_addr: "REWARD_NFT".to_string(),
            //     msg: to_binary(&Cw721ExecuteMsg::TransferNft {
            //         recipient: "creator".to_string(),
            //         token_id: "74".to_string(),
            //     })
            //     .unwrap(),
            //     funds: vec![],
            // }),
            // CosmosMsg::Wasm(WasmMsg::Execute {
            //     contract_addr: "REWARD_NFT".to_string(),
            //     msg: to_binary(&Cw721ExecuteMsg::TransferNft {
            //         recipient: "creator".to_string(),
            //         token_id: "50".to_string(),
            //     })
            //     .unwrap(),
            //     funds: vec![],
            // }),
            // CosmosMsg::Wasm(WasmMsg::Execute {
            //     contract_addr: "REWARD_NFT".to_string(),
            //     msg: to_binary(&Cw721ExecuteMsg::TransferNft {
            //         recipient: "creator".to_string(),
            //         token_id: "4".to_string(),
            //     })
            //     .unwrap(),
            //     funds: vec![],
            // }),
        ])
        .add_attributes(vec![
            attr("action", "claim_reward"),
            attr("reward_num", "1"),
        ]);
    assert_eq!(res, expected_res);
}

//
// #[test]
// fn test_staking_time() {
//     let mut token_list = vec![];
//
//     token_list.push(TokenInfo {
//         token_kind: 0,
//         token_id: "1".to_string(),
//         is_common: true,
//     });
//
//     token_list.push(TokenInfo {
//         token_kind: 0,
//         token_id: "2".to_string(),
//         is_common: true,
//     });
//     let expected_sec = 432 * 1*9 / (2*10) ; // 86400 * 84 /2 * 0.9
//
//     let reward_time = staking_time(&token_list).unwrap();
//     assert_eq!(expected_sec, reward_time);
//
//     let mut token_list = vec![];
//
//     token_list.push(TokenInfo {
//         token_kind: 1,
//         token_id: "1".to_string(),
//         is_common: true,
//     });
//
//     token_list.push(TokenInfo {
//         token_kind: 1,
//         token_id: "2".to_string(),
//         is_common: true,
//     });
//     let expected_sec = 432 * 2 * 9 / (2*10); // 86400 * 168 /2 * 0.9
//     let reward_time = staking_time(&token_list).unwrap();
//     assert_eq!(expected_sec, reward_time);
//
//     let mut token_list = vec![];
//
//     token_list.push(TokenInfo {
//         token_kind: 0,
//         token_id: "1".to_string(),
//         is_common: true,
//     });
//
//     token_list.push(TokenInfo {
//         token_kind: 1,
//         token_id: "2".to_string(),
//         is_common: true,
//     });
//     let expected_sec = 432 * 3*9 / 40; // 86400 * (84/2 + 168/2)/2 * 0.9
//     let reward_time = staking_time(&token_list).unwrap();
//     assert_eq!(expected_sec, reward_time);
//
//     let mut token_list = vec![];
//
//     token_list.push(TokenInfo {
//         token_kind: 0,
//         token_id: "1".to_string(),
//         is_common: true,
//     });
//
//     token_list.push(TokenInfo {
//         token_kind: 1,
//         token_id: "2".to_string(),
//         is_common: false,
//     });
//     token_list.push(TokenInfo {
//         token_kind: 1,
//         token_id: "3".to_string(),
//         is_common: false,
//     });
//
//     let expected_sec = (432 * 8* 5 /3)/30; // 86400 * (84/3 + 168*2/3)/3 * 0.8
//     let reward_time = staking_time(&token_list).unwrap();
//     assert_eq!(expected_sec, reward_time);
// }
