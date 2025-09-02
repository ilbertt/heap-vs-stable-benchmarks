use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

use crate::cbor::asset::CborAsset;
use crate::store::Store;

const MEMORY_ID: u8 = 1;

type Memory = VirtualMemory<DefaultMemoryImpl>;

pub struct CborStableState {
    assets: StableBTreeMap<String, CborAsset, Memory>,
}

impl Default for CborStableState {
    fn default() -> Self {
        Self {
            assets: StableBTreeMap::init(
                MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(MEMORY_ID))),
            ),
        }
    }
}

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    // Initialize a `StableBTreeMap` with `MemoryId(0)`.
    static STABLE_STATE: RefCell<CborStableState> = RefCell::new(CborStableState::default());
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
    STABLE_STATE.with(|state| f(&mut state.borrow_mut()))
}
