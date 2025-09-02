use ic_stable_structures::StableBTreeMap;
use std::cell::RefCell;

use common::store::Store;

use crate::stable::cbor::asset::CborAsset;
use crate::stable::memory_manager::{get_cbor_memory, Memory};

pub struct CborStableState {
    assets: StableBTreeMap<String, CborAsset, Memory>,
}

impl Default for CborStableState {
    fn default() -> Self {
        Self {
            assets: StableBTreeMap::init(get_cbor_memory()),
        }
    }
}

thread_local! {
    static CBOR_STABLE_STATE: RefCell<CborStableState> = RefCell::new(CborStableState::default());
}

impl Store<CborAsset> for CborStableState {
    fn get(&self, key: String) -> Option<CborAsset> {
        self.assets.get(&key)
    }

    fn set(&mut self, key: String, asset: CborAsset) {
        self.assets.insert(key, asset);
    }
}

pub fn with_cbor_stable_state<T>(f: impl FnOnce(&mut CborStableState) -> T) -> T {
    CBOR_STABLE_STATE.with(|state| f(&mut state.borrow_mut()))
}
