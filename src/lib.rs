#![deny(clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

use crate::id::{Checksum, Id, Version};
use crate::storage::ReleaseStorage;
use blake2::{Blake2s256, Digest};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, require, AccountId};

mod id;
mod storage;

pub type Result<T> = std::result::Result<T, error::Error>;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Release data stored in the database.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ReleaseData(Vec<u8>);

#[near_bindgen]
pub struct State {
    storage: ReleaseStorage,
    owner_id: AccountId,
}

#[near_bindgen]
impl State {
    #[must_use]
    #[init]
    pub fn new(owner: AccountId) -> Self {
        Self {
            storage: ReleaseStorage::default(),
            owner_id: owner,
        }
    }

    #[must_use]
    pub fn is_owner(&self) -> bool {
        env::predecessor_account_id() == self.owner_id
    }

    /// Pushes a new release of the contract into the storage.
    pub fn push(&mut self, version: String, code: Vec<u8>, latest: bool) -> Vec<u8> {
        require!(self.is_owner(), "Access denied: owner's method");

        let mut hasher = Blake2s256::default();
        hasher.update(&code);
        let checksum: Vec<u8> = hasher.finalize().to_vec();
        let id = {
            let version = Version::try_from(version).unwrap();
            Id::new(version, Checksum(checksum.clone()))
        };
        self.storage.insert(id, &ReleaseData(code), latest);
        checksum
    }

    // Yanks a release from the storage with a provided ID.
    // pub fn pull(&self, id: String) -> Result {
    //     let id = Id::try_from(id);
    //     let data = self.storage.remove()
    //     self.storage.remove(id).expect("ERR1_")
    //     todo!()
    //     return self.storage.data.clone();
    // }

    /// Yanks a release from the storage.
    pub fn yank(&self) {
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Owner's method"
        );
        todo!()
        // let key = self.storage.checksum.as_bytes().to_vec();
        // env::storage_remove(&key);
        // self.storage = ChecksumStorage {
        //     owner: self.storage.owner.clone(),
        //     ..Default::default()
        // };
    }

    // Lists all releases.
    // pub fn list(&self) -> String {
    //     todo!()
    //     return self.storage.checksum.clone();
    // }

    // pub fn latest(&self) -> String {
    //     todo!()
    // }
}

mod error {
    use crate::id::error::IdError;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error(transparent)]
        IdError(#[from] IdError),
    }
}
