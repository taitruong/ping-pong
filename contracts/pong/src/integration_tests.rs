#[cfg(test)]
mod tests {
    use crate::helpers::PongTemplateContract;
    use crate::msg::InstantiateMsg;
    use cosmwasm_std::{Addr, Coin, Empty, Uint128};
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    pub fn pong_contract_template() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
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

    fn proper_instantiate() -> (App, PongTemplateContract) {
        let mut app = mock_app();
        let pong_code_id = app.store_code(pong_contract_template());

        let msg = InstantiateMsg { count: 1i32 };
        let pong_template_contract_addr = app
            .instantiate_contract(
                pong_code_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "test",
                None,
            )
            .unwrap();

        let pong_template_contract = PongTemplateContract(pong_template_contract_addr);

        (app, pong_template_contract)
    }

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn count() {
            let (mut app, pong_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::Increment {};
            let cosmos_msg = pong_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(USER), cosmos_msg).unwrap();
        }
    }
}
