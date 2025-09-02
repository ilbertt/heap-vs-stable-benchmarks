use std::{
    cell::RefCell,
    io::{Read, Write},
};

use crate::store::Store;

#[derive(Default)]
pub struct StableState {
    bytes_size: usize,
}

thread_local! {
    static STABLE_STATE: RefCell<StableState> = RefCell::new(StableState::default());
}

impl Store<Vec<u8>> for StableState {
    fn get(&self) -> Vec<u8> {
        let mut bytes = vec![0u8; self.bytes_size];
        let mut reader = ic_cdk::stable::StableReader::default();
        reader.read_exact(&mut bytes).unwrap();
        bytes
    }

    fn set(&mut self, bytes: Vec<u8>) {
        let mut writer = ic_cdk::stable::StableWriter::default();
        writer.write_all(&bytes).unwrap();
        self.bytes_size = bytes.len();
    }
}

pub fn with_stable_state<T>(f: impl FnOnce(&mut StableState) -> T) -> T {
    STABLE_STATE.with(|state| f(&mut state.borrow_mut()))
}
