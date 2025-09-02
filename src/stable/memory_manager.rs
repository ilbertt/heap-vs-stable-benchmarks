use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl,
};

pub type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
}

const CANDID_STATE_MEMORY_ID: u8 = 0;
const CBOR_STATE_MEMORY_ID: u8 = 1;

pub fn get_candid_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(CANDID_STATE_MEMORY_ID)))
}

pub fn get_cbor_memory() -> Memory {
    MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(CBOR_STATE_MEMORY_ID)))
}
