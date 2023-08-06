#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response, StdError,
    StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:cw-ibc-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    
    CONFIG.save(deps.storage, &Config{
        val: 0,
    })?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        // ExecuteMsg::Increment { } => {
        //     Ok(Response::new())
        // }
    }
}

// sudo msg
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {        
        SudoMsg::JunoBeginBlock { } => {

            // load config, the increase +1
            let mut config = CONFIG.load(deps.storage)?;
            config.val -=1;
            CONFIG.save(deps.storage, &config)?;

            Ok(Response::new())
        }
        SudoMsg::JunoEndBlock { } => {

            // load config, the increase +1
            let mut config = CONFIG.load(deps.storage)?;
            config.val += 1;
            CONFIG.save(deps.storage, &config)?;

            Ok(Response::new())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig { } => to_binary(&query_config(deps)?),        
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let count = CONFIG.load(deps.storage)?.val;
    Ok(Config { val: count })
}
