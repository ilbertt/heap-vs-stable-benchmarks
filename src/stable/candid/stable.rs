use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

use crate::stable::candid::asset::CandidAsset;
use crate::stable::memory_manager::{get_candid_memory, Memory};
use crate::store::Store;

pub struct CandidStableState {
    assets: StableBTreeMap<String, CandidAsset, Memory>,
}

impl Default for CandidStableState {
    fn default() -> Self {
        Self {
            assets: StableBTreeMap::init(get_candid_memory()),
        }
    }
}

thread_local! {
    static STABLE_STATE: RefCell<CandidStableState> = RefCell::new(CandidStableState::default());
}

impl Store<CandidAsset> for CandidStableState {
    fn get(&self, key: String) -> Option<CandidAsset> {
        self.assets.get(&key)
    }

    fn set(&mut self, key: String, asset: CandidAsset) {
        self.assets.insert(key, asset);
    }
}

pub fn with_candid_stable_state<T>(f: impl FnOnce(&mut CandidStableState) -> T) -> T {
    STABLE_STATE.with(|state| f(&mut state.borrow_mut()))
}
