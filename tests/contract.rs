use crate::utils::TestContract;
use near_sdk::ONE_YOCTO;

mod utils;

#[tokio::test]
async fn test_push() {
    let contract = TestContract::new(None).await.unwrap();

    let version = "v1.2.3";
    let code: Vec<String> = vec![];
    let latests = false;
    let res = contract
        .contract
        .call("push")
        .args_json((version, code, latests))
        .gas(10_000_000_000_000)
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    println!("{:#?}", res.clone());
    assert!(res.is_success());
}
