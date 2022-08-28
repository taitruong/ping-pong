#[cfg(test)]
mod tests {
    use crate::helpers::PingTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128, to_binary, WasmMsg};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
    use cw_utils::parse_instantiate_response_data;
    use pong::{contract as pong_contract, helpers::PongTemplateContract};

    pub fn ping_contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        ).with_reply(crate::contract::reply);
        Box::new(contract)
    }

    pub fn pong_contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            pong_contract::execute,
            pong_contract::instantiate,
            pong_contract::query,
        );
        Box::new(contract)
    }

    const USER: &str = "USER";
    const ADMIN: &str = "ADMIN";
    const NATIVE_DENOM: &str = "denom";

    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
            router
                .bank
                .init_balance(
                    storage,
                    &Addr::unchecked(USER),
                    vec![Coin {
                        denom: NATIVE_DENOM.to_string(),
                        amount: Uint128::new(1),
                    }],
                )
                .unwrap();
        })
    }

    fn proper_instantiate() -> (App, PingTemplateContract, PongTemplateContract) {
        let mut app = mock_app();
        let ping_code_id = app.store_code(ping_contract_template());
        let pong_code_id = app.store_code(pong_contract_template());

        let msg = InstantiateMsg { count: 0, pong_code_id };
        let init_msg = to_binary(&msg).unwrap();
        let msg = WasmMsg::Instantiate {
            admin: None,
            code_id: ping_code_id,
            msg: init_msg,
            funds: [].to_vec(),
            label: "test".into(),
        };

        // get ping address
        let res = app.execute(Addr::unchecked(ADMIN), msg.into()).unwrap();
        let data = parse_instantiate_response_data(res.data.clone().unwrap_or_default().as_slice()).unwrap();
        let ping_template_contract_addr = Addr::unchecked(data.contract_address);
        let ping_template_contract = PingTemplateContract(ping_template_contract_addr);

        // get pong address
        let pong_event = res.events.iter().find(|event| event.attributes.iter().find(|attr| attr.key == "pong address").is_some()).unwrap();
        let pong_attribute = pong_event.attributes.iter().find(|attr| attr.key == "pong address").unwrap();
        let pong_template_contract_addr = Addr::unchecked(pong_attribute.to_owned().value);
        let pong_template_contract = PongTemplateContract(pong_template_contract_addr);

        (app, ping_template_contract, pong_template_contract)
    }

    mod count {
        use super::*;
        use crate::msg::{ExecuteMsg, QueryMsg, GetCountResponse};
        use assert_matches::assert_matches;
        use cosmwasm_std::{QueryRequest, WasmQuery, to_binary, StdError};

        #[test]
        fn count() {
            let (mut app, ping_template_contract, _pong_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::Increment {};
            let cosmos_msg = ping_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();

            let query_msg: QueryMsg = QueryMsg::GetCount {};
            let response: Result<GetCountResponse, StdError> = app.wrap().query(&QueryRequest::Wasm(WasmQuery::Smart {
                contract_addr: ping_template_contract.addr().to_string(),
                msg: to_binary(&query_msg).unwrap()
            }));
            assert_matches!(
                response,
                Ok(GetCountResponse{count}) if count == 1);
        }
    }
}
