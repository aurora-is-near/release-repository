use crate::utils::{CustomId, CustomIdStatus, TestContract};
use aurora_release_repository::id::{Checksum, Id, Status, Version};
use near_sdk::env::sha256;
use near_sdk::json_types::Base64VecU8;

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

    let res = contract.push(version, &code, latest, 6).await.unwrap();
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

    let res = contract.push(version, &code, latest, 10).await.unwrap();
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

    let res = contract.push(version, &code, latest, 6).await.unwrap();
    assert!(res.is_success());

    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res: CustomIdStatus = contract.get_status(&id).await.unwrap();
    assert_eq!(res.status, Status::Released);
    assert_eq!(res.id.version, version);
    assert_eq!(res.id.checksum, checksum.to_string());

    let res = contract.pull(&id).await.unwrap();
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

    let res = contract.push(version, &code, latest, 6).await.unwrap();
    assert!(res.is_success());
    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res = contract.get_blob(&id).await.unwrap();
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

    let res = contract.push(version, &code, latest, 6).await.unwrap();
    assert!(res.is_success());
    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res = contract.get_blob(&id).await.unwrap();
    assert_eq!(res, code);

    //== Release 3
    let code_data = vec![100, 121, 31, 20, 0, 23, 32, 3];
    let checksum = Checksum(sha256(&code_data));
    let version = "v1.2.5";
    let id = Id::new(Version::try_from(version).unwrap(), checksum.clone());
    let code = Base64VecU8(code_data);
    let latest = false;
    release_list.push(CustomIdStatus::new(id.clone(), None));

    let res = contract.push(version, &code, latest, 6).await.unwrap();
    assert!(res.is_success());
    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res = contract.get_blob(&id).await.unwrap();
    assert_eq!(res, code);

    //== List releases
    let res: Vec<CustomIdStatus> = contract.list().await.unwrap();
    for (i, status) in res.iter().enumerate() {
        assert_eq!(status, &release_list[i]);
    }

    //== Latest release
    let res: CustomId = contract.get_latest().await.unwrap();
    assert_eq!(res, release_id);
}

#[tokio::test]
async fn test_yank() {
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

    let res = contract.push(version, &code, latest, 6).await.unwrap();
    assert!(res.is_success());
    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res = contract.get_blob(&id).await.unwrap();
    assert_eq!(res, code);

    //== Release 2
    let code_data = vec![100, 121, 31, 20, 0, 23, 32, 2];
    let checksum = Checksum(sha256(&code_data));
    let version = "v1.2.4";
    let id = Id::new(Version::try_from(version).unwrap(), checksum.clone());
    let yank_id = id.clone();
    let code = Base64VecU8(code_data);
    let latest = true;
    let release_id = CustomId {
        version: version.to_string(),
        checksum: checksum.to_string(),
    };
    release_list.push(CustomIdStatus::new(id.clone(), None));

    let res = contract.push(version, &code, latest, 6).await.unwrap();
    assert!(res.is_success());
    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res = contract.get_blob(&id).await.unwrap();
    assert_eq!(res, code);

    //== Release 3
    let code_data = vec![100, 121, 31, 20, 0, 23, 32, 3];
    let checksum = Checksum(sha256(&code_data));
    let version = "v1.2.5";
    let id = Id::new(Version::try_from(version).unwrap(), checksum.clone());
    let code = Base64VecU8(code_data);
    let latest = false;
    release_list.push(CustomIdStatus::new(id.clone(), None));

    let res = contract.push(version, &code, latest, 6).await.unwrap();
    assert!(res.is_success());
    let res = Id::try_from(res.into_result().unwrap().json::<String>().unwrap()).unwrap();
    assert_eq!(res, id);

    let res = contract.get_blob(&id).await.unwrap();
    assert_eq!(res, code);

    //== List releases
    let res: Vec<CustomIdStatus> = contract.list().await.unwrap();
    for (i, status) in res.iter().enumerate() {
        assert_eq!(status, &release_list[i]);
    }

    //== Latest release
    let res: CustomId = contract.get_latest().await.unwrap();
    assert_eq!(res, release_id);

    //== List yank
    let res: Vec<CustomId> = contract.yank_list().await.unwrap();
    assert!(res.is_empty());

    //== Yank
    let res = contract.pull(&yank_id).await.unwrap();
    assert!(res.is_success());
    let res = res.into_result().unwrap().json::<CustomIdStatus>().unwrap();
    assert_eq!(res.status, Status::Yanked);
    assert_eq!(res.id.version, yank_id.version.to_string());
    assert_eq!(res.id.checksum, yank_id.checksum.to_string());

    //== List yank
    let res: Vec<CustomId> = contract.yank_list().await.unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].version, yank_id.version.to_string());
    assert_eq!(res[0].checksum, yank_id.checksum.to_string());

    let res: CustomIdStatus = contract.get_status(&yank_id).await.unwrap();
    assert_eq!(res.status, Status::Yanked);
    assert_eq!(res.id.version, yank_id.version.to_string());
    assert_eq!(res.id.checksum, yank_id.checksum.to_string());
}
