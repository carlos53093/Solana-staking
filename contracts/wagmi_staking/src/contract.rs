use cosmwasm_bignumber::Decimal256;
use cosmwasm_std::{
    attr, entry_point, from_binary, to_binary, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, Response, StdError, StdResult, Uint128, WasmMsg,
};
use cw721::{Cw721ExecuteMsg, Cw721ReceiveMsg};

use crate::querier::{is_common_kongz, is_common_monkeez, query_all_tokenid_from_owner};

use crate::state::{read_holder, store_holder, Cluster, Config, Holder, CONFIG};

use wagmi_protocol::staking::{
    ClusterResponse, ClusterReward, ConfigResponse, Cw721HookMsg, ExecuteMsg, InstantiateMsg,
    MigrateMsg, QueryMsg, RewardResponse, TokenInfo, TokensInfoResponse,
};

const MONKEES_ONE_DAY: u64 = 84;
const KONGZ_ONE_DAY: u64 = 168;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // if msg.legendaries_ids.len() < 3 {
    //     return Err(StdError::generic_err(
    //         "legendaries_ids must have more than 3",
    //     ));
    // }

    let config = Config {
        owner: deps.api.addr_canonicalize(info.sender.as_str())?,
        monkeez_nft: deps.api.addr_canonicalize(&msg.monkeez_nft)?,
        kongz_nft: deps.api.addr_canonicalize(&msg.kongz_nft)?,
        reward_nft: deps.api.addr_canonicalize(&msg.reward_nft)?,
        tokens_owner: deps.api.addr_canonicalize(&msg.tokens_owner)?,
        // legendaries_ids: msg.legendaries_ids,
    };
    // NEW_TOKEN_ID.save(deps.storage, &0u64)?;

    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attributes(vec![
        attr("action", "instantiate"),
        attr("owner", info.sender),
        attr("monkeez_nft", &msg.monkeez_nft),
        attr("kongz_nft", &msg.kongz_nft),
        attr("reward_nft", &msg.reward_nft),
    ]))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::ReceiveNft(msg) => receive_cw721(deps, env, info, msg),
        ExecuteMsg::Unstake {
            token_kind,
            token_id,
        } => execute_unstake(deps, env, info, token_kind, token_id),
        ExecuteMsg::ClaimReward {} => execute_claim_reward(deps, env, info),
        ExecuteMsg::Update {
            owner,
            monkeez_nft,
            kongz_nft,
            reward_token,
        } => execute_update(deps, env, info, owner, monkeez_nft, kongz_nft, reward_token),
    }
}

fn receive_cw721(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    cw721_msg: Cw721ReceiveMsg,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    match from_binary(&cw721_msg.msg) {
        Ok(Cw721HookMsg::Stake {}) => {
            //check reward_nft
            if deps.api.addr_canonicalize(info.sender.as_str())? == config.monkeez_nft {
                return execute_stake(deps, env, info, cw721_msg.sender, cw721_msg.token_id, 0u64);
            } else if deps.api.addr_canonicalize(info.sender.as_str())? == config.kongz_nft {
                return execute_stake(deps, env, info, cw721_msg.sender, cw721_msg.token_id, 1u64);
            } else {
                return Err(StdError::generic_err("unauthorized"));
            }
        }
        _ => Err(StdError::generic_err("missing stake hook")),
    }
}

pub fn execute_stake(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    sender: String,
    token_id: String,
    nft_kind: u64,
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    let sender_raw = deps.api.addr_canonicalize(&sender)?;
    let mut holder = read_holder(deps.storage, &sender_raw)?;

    // if holder.last_reward_time == 0 {
    //     holder.last_reward_time = env.block.time.seconds();
    // }

    update_reward(&mut holder, env.clone());

    let is_common = if nft_kind == 1 {
        //kongz
        is_common_kongz(deps.as_ref(), &config.kongz_nft, &token_id)?
    } else {
        // monkeez
        is_common_monkeez(deps.as_ref(), &config.monkeez_nft, &token_id)?
    };
    let token_info = TokenInfo {
        token_kind: nft_kind,
        token_id: token_id.clone(),
        is_common,
    };

    let mut is_inputed = false;
    for cluster in holder.clusters.iter_mut() {
        if cluster.token_ids.len() < 5 {
            cluster.token_ids.push(token_info.clone());
            is_inputed = true;
            break;
        }
    }

    if !is_inputed {
        let mut cluster = Cluster {
            token_ids: vec![],
            last_reward_time: env.block.time.seconds(),
            last_reward_earned: Decimal::zero(),
            last_reward_release: Decimal::zero(),
        };
        cluster.token_ids.push(token_info.clone());
        holder.clusters.push(cluster);
    }

    store_holder(deps.storage, &sender_raw, &holder)?;

    Ok(Response::new().add_attributes(vec![
        attr("action", "stake"),
        attr("token_kind", nft_kind.to_string()),
        attr("token_id", token_id.clone()),
    ]))
}

pub fn execute_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    token_kind: u64,
    token_id: String,
) -> StdResult<Response> {
    if token_kind >= 2 {
        return Err(StdError::generic_err("token_kind has only 0 or 1"));
    }
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    let mut holder = read_holder(deps.storage, &sender_raw)?;

    let mut is_staked = false;
    update_reward(&mut holder, env);
    for cluster in holder.clusters.iter_mut() {
        let staked_nft_option = cluster
            .token_ids
            .iter()
            .find(|&x| x.is_match(token_kind, &token_id));
        if staked_nft_option.is_some() {
            is_staked = true;
            cluster
                .token_ids
                .retain(|x| !x.is_match(token_kind, &token_id));

            break;
        }
    }

    if !is_staked {
        return Err(StdError::generic_err("Sender must have staked tokenID"));
    }

    // update_reward(&mut holder, env);
    //
    // holder
    //     .token_ids
    //     .retain(|x| !x.is_match(token_kind, &token_id));
    store_holder(deps.storage, &sender_raw, &holder)?;
    //transfer
    let config = CONFIG.load(deps.storage)?;
    let nft_addr = match token_kind {
        0 => deps.api.addr_humanize(&config.monkeez_nft)?,
        1 => deps.api.addr_humanize(&config.kongz_nft)?,
        _ => {
            return Err(StdError::generic_err("Invalid token_kind"));
        }
    };
    Ok(Response::new()
        .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: nft_addr.to_string(),
            msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                recipient: info.sender.to_string(),
                token_id: token_id.clone(),
            })?,
            funds: vec![],
        })])
        .add_attributes(vec![
            attr("action", "unstake"),
            attr("receiver", info.sender.as_str()),
            attr("token_id", token_id.as_str()),
            attr("token_kind", token_kind.to_string().as_str()),
        ]))
}

pub fn execute_claim_reward(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    let sender_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    let mut holder = read_holder(deps.storage, &sender_raw)?;
    update_reward(&mut holder, env.clone());
    let mut total_mint_num = Uint128::zero();
    for cluster in holder.clusters.iter_mut() {
        let release_reward = cluster.last_reward_earned - cluster.last_reward_release;
        let mint_num = Uint128::from(1u128) * release_reward;
        if mint_num > Uint128::zero() {
            cluster.last_reward_release =
                cluster.last_reward_release + Decimal::from_ratio(mint_num, Uint128::from(1u128));
        }
        total_mint_num += mint_num;
    }
    let mut msgs = vec![];
    if total_mint_num > Uint128::zero() {
        // holder.last_reward_release =
        //     holder.last_reward_release + Decimal::from_ratio(mint_num, Uint128::from(1u128));
        // mint

        // let mut new_token_id = NEW_TOKEN_ID.load(deps.storage)?;

        let reward_nft_addr = deps.api.addr_humanize(&config.reward_nft)?.to_string();
        let reward_token_owner = deps.api.addr_humanize(&config.tokens_owner)?.to_string();
        let token_ids = query_all_tokenid_from_owner(
            deps.as_ref(),
            reward_nft_addr.clone(),
            reward_token_owner,
        )?;

        // let legendary_ids = config.legendaries_ids;
        // let mut k = 0;
        // if total_mint_num > Uint128::from(30u128) {
        //     total_mint_num = Uint128::from(30u128);
        // };
        let selected_token_ids: Vec<String> =
            get_selected_random(token_ids, total_mint_num.u128() as u64, env.clone());
        for x in selected_token_ids.into_iter() {
            let token_id = x;
            // new_token_id += 1;
            msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: reward_nft_addr.clone(),
                msg: to_binary(&Cw721ExecuteMsg::TransferNft {
                    recipient: info.sender.to_string(),
                    token_id,
                })?,
                funds: vec![],
            }));
        }
        // NEW_TOKEN_ID.save(deps.storage, &new_token_id)?;
    }
    store_holder(deps.storage, &sender_raw, &holder)?;
    Ok(Response::new().add_messages(msgs).add_attributes(vec![
        attr("action", "claim_reward"),
        attr("reward_num", total_mint_num),
    ]))
}


fn execute_update(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    owner: Option<String>,
    monkeez_nft: Option<String>,
    kongz_nft: Option<String>,
    reward_token: Option<String>,
) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.owner != deps.api.addr_canonicalize(info.sender.as_str())? {
        return Err(StdError::generic_err("unauthorized"));
    }

    let mut attr_vec = vec![];
    attr_vec.push(attr("action", "update"));

    if let Some(owner) = owner {
        config.owner = deps.api.addr_canonicalize(owner.as_str())?;
        attr_vec.push(attr("owner", owner));
    }
    if let Some(monkeez_nft) = monkeez_nft {
        config.monkeez_nft = deps.api.addr_canonicalize(monkeez_nft.as_str())?;
        attr_vec.push(attr("monkeez_nft", monkeez_nft));
    }
    if let Some(kongz_nft) = kongz_nft {
        config.kongz_nft = deps.api.addr_canonicalize(kongz_nft.as_str())?;
        attr_vec.push(attr("kongz_nft", kongz_nft));
    }
    if let Some(reward_token) = reward_token {
        config.reward_nft = deps.api.addr_canonicalize(reward_token.as_str())?;
        attr_vec.push(attr("reward_token", reward_token));
    }
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new().add_attributes(attr_vec))
}

fn update_reward(holder: &mut Holder, env: Env) {
    // let mut reward: Decimal = Decimal::zero();
    for cluster in holder.clusters.iter_mut() {
        if cluster.token_ids.len() == 0 {
            continue;
        }

        let secs_need_reward = need_staking_time_for_reward(cluster).unwrap();
        let diff_sec = env.block.time.seconds() - cluster.last_reward_time;
        let reward = Decimal::from_ratio(Uint128::from(diff_sec), Uint128::from(secs_need_reward));
        cluster.last_reward_time = env.block.time.seconds();
        cluster.last_reward_earned = cluster.last_reward_earned + reward;
    }

    // holder.last_reward_time = env.block.time.seconds();
    // holder.last_reward_earned = holder.last_reward_earned + reward;
}

// fn need_staking_time_for_reward(token_list: &Vec<TokenInfo>) -> Option<u64> {
fn need_staking_time_for_reward(cluster: &Cluster) -> Option<u64> {
    let token_list = &cluster.token_ids;
    let one_for_monkeez: u64 = MONKEES_ONE_DAY * 86400; //84 days
    let one_for_kongz: u64 = KONGZ_ONE_DAY * 86400; // 168 days
                                                    // let one_for_monkeez: u64 = MONKEES_ONE_DAY * 432; //7.2 min
                                                    // let one_for_kongz: u64 = KONGZ_ONE_DAY * 432; // 14.4 min

    let staked_count: u64 = token_list.len() as u64;
    if staked_count == 1 {
        return match token_list[0].token_kind {
            0 => {
                if token_list[0].is_common  {
                    return Some(one_for_monkeez);
                }else{
                    return Some(one_for_monkeez - 86400);
                }
            },
            1 => {
                if token_list[0].is_common {
                    return Some(one_for_kongz)
                } else{
                    return Some(one_for_monkeez -86400);
                }
            },
            _ => None,
        };
    }
    let monkeez_count: u64 = token_list
        .iter()
        .filter(|&x| x.token_kind == 0 || x.is_common == false)
        .count() as u64;
    let kongz_count: u64 = token_list
        .iter()
        .filter(|&x| x.token_kind == 1 && x.is_common == true)
        .count() as u64;

    // ((84 * X/staked_num+ 168 * Y/staked_num)/staked_num) * (1 - 0.1 *(staked_num -1))

    // t1 = 84*X/staked_num + 168*Y/staked_num
    let t1 = Decimal::from_ratio(
        Uint128::from(one_for_monkeez * monkeez_count),
        Uint128::from(staked_count),
    ) + Decimal::from_ratio(
        Uint128::from(one_for_kongz * kongz_count),
        Uint128::from(staked_count),
    );

    // t2 = t1 / staked_num
    let t2: Decimal256 = Decimal256::from(t1)
        / Decimal256::from(Decimal::from_ratio(
            Uint128::from(staked_count),
            Uint128::from(1u128),
        ));

    //1 - (staked_num -1) /10
    let k = Decimal::one()
        - Decimal::from_ratio(Uint128::from(staked_count - 1), Uint128::from(10u128));
    if k.is_zero() {
        return Some(86400);
    }
    let t3 = t2 * Decimal256::from(k);
    let t4 = Decimal::from(t3) * Uint128::from(1u128);
    let t5 = t4.u128() as u64;
    let t6 = t5 - 86400 * cluster.count_legendary();
    Some(t6)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        // QueryMsg::Reward { staker } => to_binary(&query_reward(deps, env, staker)?),
        QueryMsg::StakedTokens { owner } => to_binary(&query_staked_tokens(deps, env, owner)?),
        QueryMsg::Reward { staker } => to_binary(&query_cluster_reward(deps, env, staker)?),
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(ConfigResponse {
        owner: deps.api.addr_humanize(&config.owner)?.to_string(),
        monkeez_nft: deps.api.addr_humanize(&config.monkeez_nft)?.to_string(),
        kongz_nft: deps.api.addr_humanize(&config.kongz_nft)?.to_string(),
        reward_nft: deps.api.addr_humanize(&config.reward_nft)?.to_string(),
    })
}
//
// pub fn query_reward(deps: Deps, env: Env, staker: String) -> StdResult<RewardResponse> {
//     let staker_raw = deps.api.addr_canonicalize(staker.as_str())?;
//     let mut holder = read_holder(deps.storage, &staker_raw)?;
//     // panic!("--{}--",holder.last_reward_earned);
//     // panic!("--{}--",holder.last_reward_release);
//
//     update_reward(&mut holder, env);
//     let mut claimable_amount = Uint128::zero();
//     for cluster in holder.clusters.iter() {
//         claimable_amount += Uint128::from(1u128) * (cluster.last_reward_earned - cluster.last_reward_release)
//     }
//
//     Ok(RewardResponse {
//         reward_amount: Uint128::from(1u128)
//             * (holder.last_reward_earned - holder.last_reward_release),
//     })
// }

pub fn query_cluster_reward(deps: Deps, env: Env, staker: String) -> StdResult<RewardResponse> {
    let staker_raw = deps.api.addr_canonicalize(staker.as_str())?;
    let mut holder = read_holder(deps.storage, &staker_raw)?;
    update_reward(&mut holder, env);

    let mut cluster_rewards = vec![];
    let mut total_claimable_amount = 0u64;
    for cluster in holder.clusters.iter() {
        let rewardable_num =
            Uint128::from(1u128) * (cluster.last_reward_earned - cluster.last_reward_release);

        let remain_decimal = cluster.last_reward_earned
            - cluster.last_reward_release
            - Decimal::from_ratio(rewardable_num, Uint128::from(1u128));

        let remain_time = if cluster.token_ids.len() > 0 {
            Some(
                (Uint128::from(need_staking_time_for_reward(cluster).unwrap())
                    * (Decimal::one() - remain_decimal))
                    .u128() as u64,
            )
        } else {
            None
        };

        cluster_rewards.push(ClusterReward {
            claimable_num: rewardable_num.u128() as u64,
            remain_time,
        });
        total_claimable_amount += rewardable_num.u128() as u64;
    }
    Ok(RewardResponse {
        claimable_amount: total_claimable_amount,
        cluster_rewards,
    })
}

pub fn query_staked_tokens(deps: Deps, _env: Env, owner: String) -> StdResult<TokensInfoResponse> {
    let owner_raw = deps.api.addr_canonicalize(owner.as_str())?;
    let holder = read_holder(deps.storage, &owner_raw)?;

    let mut clusters = vec![];
    for cluster in holder.clusters.iter() {
        clusters.push(ClusterResponse {
            tokens: cluster.token_ids.clone(),
        });
    }
    Ok(TokensInfoResponse { clusters })
}

fn get_selected_random(mut token_ids: Vec<String>, num: u64, env: Env) -> Vec<String> {
    let time = env.block.time.seconds();
    let mut sel_token_ids: Vec<String> = vec![];
    let len = token_ids.len();
    for i in 0..num {
        let k = time * (i + 1) % (len - i as usize) as u64;
        sel_token_ids.push(token_ids[k as usize].clone());
        token_ids.remove(k as usize);
    }
    sel_token_ids
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
