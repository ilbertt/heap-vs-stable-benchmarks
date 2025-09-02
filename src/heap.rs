use std::{cell::RefCell, collections::BTreeMap};

use crate::{asset::Asset, store::Store};

#[derive(Default)]
pub struct HeapState {
    assets: BTreeMap<String, Asset>,
}

thread_local! {
    static HEAP_STATE: RefCell<HeapState> = RefCell::new(HeapState::default());
}

impl Store<Asset> for HeapState {
    fn get(&self, key: String) -> Option<Asset> {
        self.assets.get(&key).cloned()
    }

    fn set(&mut self, key: String, asset: Asset) {
        self.assets.insert(key, asset);
    }
}

pub fn with_heap_state<T>(f: impl FnOnce(&mut HeapState) -> T) -> T {
    HEAP_STATE.with(|state| f(&mut state.borrow_mut()))
}
