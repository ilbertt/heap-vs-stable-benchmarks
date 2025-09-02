mod asset;
mod heap;
mod rand;
mod stable;
mod store;

#[cfg(feature = "canbench-rs")]
mod bench {
    use std::hint::black_box;

    use canbench_rs::{bench, BenchResult};

    use crate::{
        asset::Asset,
        heap::state::with_heap_state,
        rand::random_bytes,
        stable::{
            candid::{asset::CandidAsset, state::with_candid_stable_state},
            cbor::{asset::CborAsset, state::with_cbor_stable_state},
        },
        store::Store,
    };

    const ASSET_TEST_KEY: &str = "test";

    const ONE_KB: usize = 1024;
    const TEN_KB: usize = 10 * ONE_KB;
    const HUNDRED_KB: usize = 100 * ONE_KB;
    const ONE_MB: usize = ONE_KB * ONE_KB;
    const TWO_MB: usize = 2 * ONE_KB * ONE_KB;

    #[bench(raw)]
    fn heap_read_0b() -> BenchResult {
        with_heap_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                Asset::new_with_content(random_bytes(0)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn heap_read_1kb() -> BenchResult {
        with_heap_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                Asset::new_with_content(random_bytes(ONE_KB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn heap_read_10kb() -> BenchResult {
        with_heap_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                Asset::new_with_content(random_bytes(TEN_KB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn heap_read_100kb() -> BenchResult {
        with_heap_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                Asset::new_with_content(random_bytes(HUNDRED_KB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn heap_read_1mb() -> BenchResult {
        with_heap_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                Asset::new_with_content(random_bytes(ONE_MB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn heap_read_2mb() -> BenchResult {
        with_heap_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                Asset::new_with_content(random_bytes(TWO_MB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_candid_read_0b() -> BenchResult {
        with_candid_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CandidAsset::new_with_content(random_bytes(0)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_candid_read_1kb() -> BenchResult {
        with_candid_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CandidAsset::new_with_content(random_bytes(ONE_KB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_candid_read_10kb() -> BenchResult {
        with_candid_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CandidAsset::new_with_content(random_bytes(TEN_KB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_candid_read_100kb() -> BenchResult {
        with_candid_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CandidAsset::new_with_content(random_bytes(HUNDRED_KB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_candid_read_1mb() -> BenchResult {
        with_candid_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CandidAsset::new_with_content(random_bytes(ONE_MB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_candid_read_2mb() -> BenchResult {
        with_candid_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CandidAsset::new_with_content(random_bytes(TWO_MB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_cbor_read_0b() -> BenchResult {
        with_cbor_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CborAsset::new_with_content(random_bytes(0)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_cbor_read_1kb() -> BenchResult {
        with_cbor_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CborAsset::new_with_content(random_bytes(ONE_KB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_cbor_read_10kb() -> BenchResult {
        with_cbor_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CborAsset::new_with_content(random_bytes(TEN_KB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_cbor_read_100kb() -> BenchResult {
        with_cbor_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CborAsset::new_with_content(random_bytes(HUNDRED_KB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_cbor_read_1mb() -> BenchResult {
        with_cbor_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CborAsset::new_with_content(random_bytes(ONE_MB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }

    #[bench(raw)]
    fn stable_cbor_read_2mb() -> BenchResult {
        with_cbor_stable_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                CborAsset::new_with_content(random_bytes(TWO_MB)),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }
}
