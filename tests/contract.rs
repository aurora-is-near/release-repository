use crate::utils::TestContract;
use aurora_release_repository::id::{Checksum, Id, Version};
use near_sdk::env::sha256;
use near_sdk::json_types::Base64VecU8;
use near_sdk::ONE_YOCTO;
use serde_json::json;

mod utils;

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
