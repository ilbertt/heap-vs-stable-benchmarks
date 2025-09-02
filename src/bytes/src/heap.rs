use std::cell::RefCell;

use crate::store::Store;

#[derive(Default)]
pub struct HeapState {
    bytes: Vec<u8>,
}

thread_local! {
    static HEAP_STATE: RefCell<HeapState> = RefCell::new(HeapState::default());
}

impl Store<Vec<u8>> for HeapState {
    fn get(&self) -> Vec<u8> {
        self.bytes.clone()
    }

    fn set(&mut self, bytes: Vec<u8>) {
        self.bytes = bytes;
    }
}

pub fn with_heap_state<T>(f: impl FnOnce(&mut HeapState) -> T) -> T {
    HEAP_STATE.with(|state| f(&mut state.borrow_mut()))
}
