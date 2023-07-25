use crate::utils::TestContract;

pub mod utils;

#[tokio::test]
async fn test_ft_transfer() {
    let _contract = TestContract::new().await.unwrap();
    /*
    let res = contract
        .contract
        .call("ft_transfer")
        .args_json((&receiver_id, transfer_amount, "transfer memo"))
        .gas(DEFAULT_GAS)
        .deposit(ONE_YOCTO)
        .transact()
        .await
        .unwrap();
    assert!(res.is_success());
    */
}
