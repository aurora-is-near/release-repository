use crate::utils::TestContract;
use aurora_release_repository::id::{Checksum, Id, Status, Version};
use near_sdk::env::sha256;
use near_sdk::json_types::Base64VecU8;
use near_sdk::serde::Deserialize;
use near_sdk::ONE_YOCTO;
use serde_json::json;

mod utils;

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
struct CustomId {
    version: String,
    checksum: String,
}

#[derive(Debug, Clone, Deserialize, Eq, PartialEq)]
struct CustomIdStatus {
    id: CustomId,
    status: Status,
}

impl CustomIdStatus {
    fn new(id: Id, status: Option<Status>) -> Self {
        Self {
            id: CustomId {
                version: id.version.to_string(),
                checksum: id.checksum.to_string(),
            },
            status: status.unwrap_or(Status::Released),
        }
    }
}

#[tokio::test]
async fn test_push() {
    let contract = TestContract::new(None).await.unwrap();

    let code_data = vec![100, 121, 31, 20, 0, 23, 32];
    let checksum = Checksum(sha256(&code_data));
    let version = "v1.2.3";
    let id = Id::new(Version::try_from(version).unwrap(), checksum);
    let code = Base64VecU8(code_data);
    let latest = false;

    let res = contract
        .contract
        .call("push")
        .args_json(json!({
            "version": version,
            "code": &code,
            "latest": latest
        }))
        .gas(6_000_000_000_000)
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);
}

#[tokio::test]
async fn test_push_2mb() {
    let contract = TestContract::new(None).await.unwrap();

    // 2 mega bytes
    let code_data: Vec<u8> = (0..20 * 1024).map(|_| 0xFF).collect();
    let checksum = Checksum(sha256(&code_data));
    let version = "v1.2.3";
    let id = Id::new(Version::try_from(version).unwrap(), checksum);
    let code = Base64VecU8(code_data);
    let latest = false;

    let res = contract
        .contract
        .call("push")
        .args_json(json!({
            "version": version,
            "code": &code,
            "latest": latest
        }))
        .gas(9_100_000_000_000)
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);
}

#[tokio::test]
async fn test_pull() {
    let contract = TestContract::new(None).await.unwrap();

    let code_data = vec![100, 121, 31, 20, 0, 23, 32];
    let checksum = Checksum(sha256(&code_data));
    let version = "v1.2.3";
    let id = Id::new(Version::try_from(version).unwrap(), checksum.clone());
    let code = Base64VecU8(code_data);
    let latest = false;

    let res = contract
        .contract
        .call("push")
        .args_json(json!({
            "version": version,
            "code": &code,
            "latest": latest
        }))
        .gas(6_000_000_000_000)
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());

    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res: CustomIdStatus = contract
        .contract
        .view("get_status")
        .args_json(json!({
            "id": id.to_string(),
        }))
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(res.status, Status::Released);
    assert_eq!(res.id.version, version);
    assert_eq!(res.id.checksum, checksum.to_string());

    let res = contract
        .contract
        .call("pull")
        .args_json(json!({
            "id": id.to_string(),
        }))
        .gas(6_000_000_000_000)
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let res = res.into_result().unwrap().json::<CustomIdStatus>().unwrap();
    assert_eq!(res.status, Status::Yanked);
    assert_eq!(res.id.version, version);
    assert_eq!(res.id.checksum, checksum.to_string());
}

#[tokio::test]
async fn test_push_and_list_check_id_and_blob() {
    let contract = TestContract::new(None).await.unwrap();

    let mut release_list: Vec<CustomIdStatus> = vec![];
    //== Release 1
    let code_data = vec![100, 121, 31, 20, 0, 23, 32, 1];
    let checksum = Checksum(sha256(&code_data));
    let version = "v1.2.3";
    let id = Id::new(Version::try_from(version).unwrap(), checksum.clone());
    let code = Base64VecU8(code_data);
    let latest = false;
    release_list.push(CustomIdStatus::new(id.clone(), None));

    let res = contract
        .contract
        .call("push")
        .args_json(json!({
            "version": version,
            "code": &code,
            "latest": latest
        }))
        .gas(6_000_000_000_000)
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res: Base64VecU8 = contract
        .contract
        .view("get_blob")
        .args_json(json!({"id": id.to_string()}))
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(res, code);

    //== Release 2
    let code_data = vec![100, 121, 31, 20, 0, 23, 32, 2];
    let checksum = Checksum(sha256(&code_data));
    let version = "v1.2.4";
    let id = Id::new(Version::try_from(version).unwrap(), checksum.clone());
    let code = Base64VecU8(code_data);
    let latest = true;
    let release_id = CustomId {
        version: version.to_string(),
        checksum: checksum.to_string(),
    };
    release_list.push(CustomIdStatus::new(id.clone(), None));

    let res = contract
        .contract
        .call("push")
        .args_json(json!({
            "version": version,
            "code": &code,
            "latest": latest
        }))
        .gas(6_000_000_000_000)
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res: Base64VecU8 = contract
        .contract
        .view("get_blob")
        .args_json(json!({"id": id.to_string()}))
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(res, code);

    //== Release 3
    let code_data = vec![100, 121, 31, 20, 0, 23, 32, 3];
    let checksum = Checksum(sha256(&code_data));
    let version = "v1.2.5";
    let id = Id::new(Version::try_from(version).unwrap(), checksum.clone());
    let code = Base64VecU8(code_data);
    let latest = false;
    release_list.push(CustomIdStatus::new(id.clone(), None));

    let res = contract
        .contract
        .call("push")
        .args_json(json!({
            "version": version,
            "code": &code,
            "latest": latest
        }))
        .gas(6_000_000_000_000)
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res: Base64VecU8 = contract
        .contract
        .view("get_blob")
        .args_json(json!({"id": id.to_string()}))
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(res, code);

    //== List releases
    let res: Vec<CustomIdStatus> = contract
        .contract
        .view("list")
        .await
        .unwrap()
        .json()
        .unwrap();
    for (i, status) in res.iter().enumerate() {
        assert_eq!(status, &release_list[i]);
    }

    //== Latest release
    let res: CustomId = contract
        .contract
        .view("latest")
        .await
        .unwrap()
        .json()
        .unwrap();
    assert_eq!(res, release_id);
}
