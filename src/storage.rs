use crate::id::IdStatus;
use crate::{id::Id, Data, Release};
use near_sdk::collections::{LookupMap, Vector};

const DATA_KEY_PREFIX: &[u8] = &[0x0];
const DATA_LIST_PREFIX: &[u8] = &[0x1];

/// Wrapper over NEAR `LookupMap` to insert, get and remove ids to data.
pub struct ReleaseStorage {
    map: LookupMap<Id, Release>,
    list: Vector<IdStatus>,
    // yanked_list: Vector<Id>,
}

#[allow(dead_code)]
impl ReleaseStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, id: Id, code: Data) {
        let release = Release::Ok(code);
        self.map.insert(&id, &release);
        let id_status = IdStatus { id, yanked: false };
        self.list.push(&id_status);
    }

    pub fn remove(&mut self, id: Id) {
        self.map.remove(&id);

        let mut i = 0;
        let mut found = false;
        for id_status in self.list.iter() {
            if id_status.id == id {
                found = true;
                break;
            }
            i += 1;
        }
        if found {
            let id_status = IdStatus { id, yanked: true };
            self.list.replace(i, &id_status);
        }
    }

    pub fn get(&self, id: &Id) -> Option<Release> {
        self.map.get(id)
    }
}

impl Default for ReleaseStorage {
    fn default() -> Self {
        Self {
            map: LookupMap::new(DATA_KEY_PREFIX),
            list: Vector::new(DATA_LIST_PREFIX),
        }
    }
}
