use std::{cell::RefCell, io::Write};

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
        let size = self.bytes_size;
        let mut vec = Vec::with_capacity(size);
        ic0::stable64_read_uninit(&mut vec.spare_capacity_mut()[..size], 0);
        // SAFETY: ic0::stable64_read writes to all of `vec[0..size]`, so `set_len` is safe to call with the new size.
        unsafe {
            vec.set_len(size);
        }
        vec
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
