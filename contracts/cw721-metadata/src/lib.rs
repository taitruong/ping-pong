use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Empty;
use cw2::set_contract_version;
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};

// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw721-metadata-onchain";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

pub type Extension = Option<Metadata>;

pub type Cw721MetadataContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let res = Cw721MetadataContract::default().instantiate(deps.branch(), env, info, msg)?;
        // Explicitly set contract name and version, otherwise set to cw721-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721MetadataContract::default().query(deps, env, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::{testing::{mock_dependencies, mock_env, mock_info}, MessageInfo, DepsMut};
    use cw721::{Cw721Query};
    use cw721_base::{ExecuteMsg};

    const CREATOR: &str = "creator";

    fn instantiate_contract<'a>(deps: DepsMut, info: MessageInfo) -> Cw721MetadataContract<'a> {
        let contract = Cw721MetadataContract::default();

        // instantiate contract
        let init_msg = InstantiateMsg {
            name: "Ark NFT Multichain".to_string(),
            symbol: "Ark Protocol".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps, mock_env(), info.clone(), init_msg)
            .unwrap();
        contract
    }

    fn mint<'a>(token_id: String, owner: String, contract: &Cw721MetadataContract, deps: DepsMut, info: MessageInfo) -> MintMsg<Extension> {
        let mint_msg = MintMsg {
            token_id: token_id.to_string(),
            owner,
            token_uri: Some("https://foo.bar".into()),
            extension: Some(Metadata {
                description: Some("Ark NFT available on any IBC chain".into()),
                name: Some("Ark NFT #0001".to_string()),
                ..Metadata::default()
            }),
        };
        let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps, mock_env(), info, exec_msg)
            .unwrap();

        mint_msg

    }

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_dependencies();

        // instantiate contract
        let info = mock_info(CREATOR, &[]);
        let contract = instantiate_contract(deps.as_mut(), info.clone());

        // mint
        let token_id = "0001";
        let owner = "minter";
        let mint_msg  = mint(token_id.to_string(), owner.to_string(), &contract, deps.as_mut(), info.clone());

        let nft_info = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(nft_info.token_uri, mint_msg.token_uri);
        assert_eq!(nft_info.extension, mint_msg.extension);
    }
}
