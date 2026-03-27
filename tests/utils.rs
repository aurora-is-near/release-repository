use aurora_release_repository::id::{Id, Status};
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::Deserialize;
use near_sdk::serde_json::json;
use near_workspaces::result::ExecutionFinalResult;
use near_workspaces::types::{Gas, NearToken};
use near_workspaces::{AccountId, Contract};
use std::str::FromStr;

const ONE_YOCTO: NearToken = NearToken::from_yoctonear(1);

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct CustomId {
    pub version: String,
    pub checksum: String,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
pub struct CustomIdStatus {
    pub id: CustomId,
    pub status: Status,
}

impl CustomIdStatus {
    pub fn new(id: Id, status: Option<Status>) -> Self {
        Self {
            id: CustomId {
                version: id.version.to_string(),
                checksum: id.checksum.to_string(),
            },
            status: status.unwrap_or(Status::Released),
        }
    }
}

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
        assert!(res.is_success());
        Ok(Self { contract })
    }

    pub async fn deploy_contract() -> anyhow::Result<Contract> {
        let worker = near_workspaces::sandbox()
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

    pub async fn push(
        &self,
        version: &str,
        code: &Base64VecU8,
        latest: bool,
        // Terra Gas
        tgas: u64,
    ) -> anyhow::Result<ExecutionFinalResult> {
        Ok(self
            .contract
            .call("push")
            .args_json(json!({
                "version": version,
                "code": code,
                "latest": latest
            }))
            .gas(Gas::from_gas(tgas * 1_000_000_000_000))
            .deposit(ONE_YOCTO)
            .transact()
            .await?)
    }

    pub async fn pull(&self, id: &Id) -> anyhow::Result<ExecutionFinalResult> {
        let res = self
            .contract
            .call("pull")
            .args_json(json!({
                "id": id.to_string(),
            }))
            .gas(Gas::from_gas(6_000_000_000_000))
            .deposit(ONE_YOCTO)
            .transact()
            .await?;
        Ok(res)
    }

    pub async fn get_blob(&self, id: &Id) -> anyhow::Result<Base64VecU8> {
        Ok(self
            .contract
            .view("get_blob")
            .args_json(json!({"id": id.to_string()}))
            .await?
            .json()?)
    }

    pub async fn get_latest(&self) -> anyhow::Result<CustomId> {
        Ok(self.contract.view("latest").await.unwrap().json()?)
    }

    pub async fn get_status(&self, id: &Id) -> anyhow::Result<CustomIdStatus> {
        Ok(self
            .contract
            .view("get_status")
            .args_json(json!({"id": id.to_string()}))
            .await?
            .json()?)
    }

    pub async fn list(&self) -> anyhow::Result<Vec<CustomIdStatus>> {
        Ok(self.contract.view("list").await.unwrap().json()?)
    }

    pub async fn yank_list(&self) -> anyhow::Result<Vec<CustomId>> {
        Ok(self.contract.view("yank_list").await.unwrap().json()?)
    }
}
