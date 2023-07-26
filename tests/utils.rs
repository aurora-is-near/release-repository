use std::str::FromStr;
use workspaces::{AccountId, Contract};

pub struct TestContract {
    pub contract: Contract,
}

impl TestContract {
    pub async fn new(owner_id: Option<&str>) -> anyhow::Result<TestContract> {
        let contract = Self::deploy_contract().await?;
        let owner_id: AccountId = if let Some(owner_id) = owner_id {
            AccountId::from_str(owner_id).unwrap()
        } else {
            contract.as_account().id().clone()
        };

        let res = contract
            .call("new")
            .args_json((owner_id,))
            .max_gas()
            .transact()
            .await?;
        println!("{res:#?}");
        assert!(res.is_success());

        Ok(Self { contract })
    }

    pub async fn deploy_contract() -> anyhow::Result<Contract> {
        let worker = workspaces::sandbox()
            .await
            .map_err(|err| anyhow::anyhow!("Failed init sandbox: {:?}", err))?;

        // Explicitly read contract file
        let contract_data =
            std::fs::read("target/wasm32-unknown-unknown/release/aurora_release_repository.wasm")
                .unwrap_or_else(|_| {
                    panic!(
                "Failed read contract in path: {:?} file: bin/aurora-eth-connector-test.wasm",
                std::env::current_dir().unwrap()
            )
                });

        let contract = worker.dev_deploy(&contract_data).await?;
        Ok(contract)
    }
}
