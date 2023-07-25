use near_sdk::serde_json::json;
use std::str::FromStr;
use workspaces::{result::ExecutionFinalResult, AccountId, Contract};

const CONTRACT_OWNER: &str = "owner";

pub struct TestContract {
    pub contract: Contract,
}

impl TestContract {
    pub async fn new() -> anyhow::Result<TestContract> {
        let contract = Self::deploy_contract().await?;
        let owner_id: AccountId = AccountId::from_str(CONTRACT_OWNER).unwrap();

        let res = contract
            .call("new")
            .args_json(json!({
                "owner": owner_id,
            }))
            .max_gas()
            .transact()
            .await?;
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

pub fn print_logs(res: ExecutionFinalResult) {
    for log in res.logs().iter() {
        println!("\t[LOG] {}", log);
    }
}
