#![deny(clippy::pedantic, clippy::nursery)]
#![allow(clippy::module_name_repetitions)]

use crate::id::{Checksum, Id, Version};
use crate::storage::ReleaseStorage;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::json_types::Base64VecU8;
use near_sdk::{env, near_bindgen, require, AccountId, PanicOnDefault};

mod id;
mod storage;

pub type Result<T> = std::result::Result<T, error::Error>;

/// Release data stored in the database.
#[derive(BorshSerialize, BorshDeserialize)]
pub struct ReleaseData(Vec<u8>);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct State {
    storage: ReleaseStorage,
    owner_id: AccountId,
}

#[near_bindgen]
impl State {
    #[must_use]
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            storage: ReleaseStorage::default(),
            owner_id,
        }
    }

    #[must_use]
    pub fn is_owner(&self) -> bool {
        env::predecessor_account_id() == self.owner_id
    }

    /// Pushes a new release of the contract into the storage.
    #[payable]
    pub fn push(&mut self, version: String, code: Base64VecU8, latest: bool) -> String {
        require!(self.is_owner(), "Access denied: owner's method");
        env::log_str(&format!("{code:?}"));
        let code: Vec<u8> = code.into();

        let checksum = env::sha256(&code);
        let id = {
            let version = Version::try_from(version).unwrap();
            Id::new(version, Checksum(checksum))
        };
        self.storage.insert(id.clone(), &ReleaseData(code), latest);
        id.to_string()
    }

    /// Yanks a release from the storage with a provided ID.
    #[payable]
    pub fn pull(&mut self, id: String) -> String {
        require!(self.is_owner(), "Access denied: owner's method");

        let id = Id::try_from(id).unwrap();
        self.storage.remove(&id);
        id.to_string()
    }

    /// Lists all releases.
    #[must_use]
    pub fn list(self) -> Vec<id::IdStatus> {
        self.storage.list()
    }

    /// Lists all yank releases.
    #[must_use]
    pub fn yank_list(self) -> Vec<Id> {
        self.storage.yanks()
    }

    /// Get latest version
    #[must_use]
    pub fn latest(&self) -> Option<Id> {
        self.storage.latest()
    }
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
