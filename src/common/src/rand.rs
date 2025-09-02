use fastrand::Rng;
use std::cell::RefCell;

thread_local! {
  static RNG: RefCell<Rng> = create_rng();
}

fn create_rng() -> RefCell<Rng> {
    let seed = get_seed();
    let rng = Rng::with_seed(seed);

    RefCell::new(rng)
}

/// Use this only inside canister runtime.
fn get_seed() -> u64 {
    ic_cdk::api::time()
}

fn with_rng<T>(cb: impl FnOnce(&mut Rng) -> T) -> T {
    RNG.with_borrow_mut(cb)
}

pub fn random_bytes(size: usize) -> Vec<u8> {
    with_rng(|rng| {
        let mut bytes = vec![0u8; size];
        rng.fill(&mut bytes);

        bytes
    })
}
