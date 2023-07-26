use crate::utils::TestContract;
use near_sdk::json_types::Base64VecU8;
use near_sdk::ONE_YOCTO;
use serde_json::json;

mod utils;

#[tokio::test]
async fn test_push() {
    let contract = TestContract::new(None).await.unwrap();

    let version = "v1.2.3";
    let code = Base64VecU8(vec![100, 121, 31, 20, 0, 23, 32]);
    let latest = false;
    let res = contract
        .contract
        .call("push")
        .args_json(json!({
            "version": version,
            "code": &code,
            "latest": latest
        }))
        .gas(10_000_000_000_000)
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();

    println!("{:#?}", res.clone());
    assert!(res.is_success());
    let res = res.into_result().unwrap().json::<String>().unwrap();
    println!("{res:#?}");
}
