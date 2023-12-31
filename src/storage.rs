use crate::id::IdStatus;
use crate::{id, id::Id, ReleaseData};
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::BorshStorageKey;

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    BlobData = 0x1,
    StatusList = 0x2,
    YankedList = 0x3,
}

/// Wrapper over NEAR `LookupMap` to insert, get and remove ids to data.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct ReleaseStorage {
    releases: LookupMap<Id, ReleaseData>,
    status_list: Vector<IdStatus>,
    yanked_list: Vector<Id>,
    latest: Option<Id>,
}

#[allow(dead_code)]
impl ReleaseStorage {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, id: Id, code: &ReleaseData, latest: bool) {
        self.releases.insert(&id, code);
        let id_status = IdStatus {
            id: id.clone(),
            status: id::Status::Released,
        };
        self.status_list.push(&id_status);
        if latest {
            self.latest = Some(id);
        }
    }

    pub fn remove(&mut self, id: &Id) -> Option<IdStatus> {
        self.releases.remove(id);

        let mut i = 0;
        let mut found = false;
        for id_status in self.status_list.iter() {
            if id_status.id == id.clone() {
                found = true;
                break;
            }
            i += 1;
        }
        if !found {
            return None;
        }
        let id_status = IdStatus {
            id: id.clone(),
            status: id::Status::Yanked,
        };
        self.status_list.replace(i, &id_status);
        self.yanked_list.push(id);
        Some(id_status)
    }

    #[must_use]
    pub fn get(&self, id: &Id) -> Option<ReleaseData> {
        self.releases.get(id)
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn list(self) -> Vec<IdStatus> {
        self.status_list.to_vec()
    }

    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn yanks(self) -> Vec<Id> {
        self.yanked_list.to_vec()
    }

    #[must_use]
    pub fn latest(&self) -> Option<Id> {
        self.latest.clone()
    }

    #[allow(clippy::needless_pass_by_value)]
    #[must_use]
    pub fn get_status(&self, id: Id) -> Option<IdStatus> {
        self.status_list.iter().find(|id_status| id_status.id == id)
    }
}

impl Default for ReleaseStorage {
    fn default() -> Self {
        Self {
            releases: LookupMap::new(StorageKey::BlobData),
            status_list: Vector::new(StorageKey::StatusList),
            yanked_list: Vector::new(StorageKey::YankedList),
            latest: None,
        }
    }
}
