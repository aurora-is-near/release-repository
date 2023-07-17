mod id;
mod storage;

use near_sdk::{env, near_bindgen, AccountId, require};
use serde::{Serialize, Deserialize};
use crate::id::{Id, Version};
use blake2::{Blake2s256, Digest};
use borsh::{BorshDeserialize, BorshSerialize};
use crate::storage::ReleaseStorage;

mod error {
    use thiserror::Error;
    use crate::id::error::IdError;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error(transparent)]
        IdError(#[from] IdError),
    }
}

pub type Result<T> = std::result::Result<T, error::Error>;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Data stored in the database.
pub type Data = Vec<u8>;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Release {
    Ok(Data),
    Yanked,
}

#[near_bindgen]
pub struct State {
    storage: ReleaseStorage,
    owner_id: AccountId,
}

#[near_bindgen]
impl State {
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            storage: ReleaseStorage::default(),
            owner_id: owner,
        }
    }

    pub fn is_owner(&self) -> bool {
        env::predecessor_account_id() == self.owner_id
    }

    /// Pushes a new release of the contract into the storage.
    pub fn push(&mut self, version: String, latest: bool, code: Vec<u8>) -> Vec<u8> {
        require!(env::predecessor_account_id() == self.owner_id, "Owner's method");

        let mut hasher = Blake2s256::default();
        hasher.update(&code);
        let checksum: Vec<u8> = hasher.finalize().to_vec();
        let id = {
            let version = Version::try_from(version).unwrap();
            Id::new(version, checksum.clone())
        };
        self.storage.insert(id, code);
        checksum
    }

    /// Yanks a release from the storage with a provided ID.
    pub fn pull(&self, id: String) -> Result<> {
        let id = Id::try_from(id);
        self.storage.remove(id).expect("ERR1_")
        todo!()
        // return self.storage.data.clone();
    }

    /// Yanks a release from the storage.
    pub fn yank(&mut self) {
        require!(env::predecessor_account_id() == self.owner_id, "Owner's method");
        todo!()
        // let key = self.storage.checksum.as_bytes().to_vec();
        // env::storage_remove(&key);
        // self.storage = ChecksumStorage {
        //     owner: self.storage.owner.clone(),
        //     ..Default::default()
        // };
    }

    /// Lists all releases.
    pub fn list(&self) -> String {
        todo!()
        // return self.storage.checksum.clone();
    }

    pub fn latest(&self) -> String {
        todo!()
    }
}
