mod asset;
mod heap;
mod stable;
mod store;

#[cfg(feature = "canbench-rs")]
mod bench {
    use std::hint::black_box;

    use canbench_rs::{bench, BenchResult};

    use common::bytes::{
        random_100kb, random_10kb, random_1kb, random_1mb, random_2mb, zero_bytes,
    };

    use crate::{
        asset::Asset,
        heap::state::with_heap_state,
        stable::{
            candid::{asset::CandidAsset, state::with_candid_stable_state},
            cbor::{asset::CborAsset, state::with_cbor_stable_state},
        },
        store::Store,
    };

    const ASSET_TEST_KEY: &str = "test";

    #[bench(raw)]
    fn heap_read_0b() -> BenchResult {
        with_heap_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                Asset::new_with_content(zero_bytes()),
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
                Asset::new_with_content(random_1kb()),
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
                Asset::new_with_content(random_10kb()),
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
                Asset::new_with_content(random_100kb()),
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
                Asset::new_with_content(random_1mb()),
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
                Asset::new_with_content(random_2mb()),
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
                CandidAsset::new_with_content(zero_bytes()),
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
                CandidAsset::new_with_content(random_1kb()),
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
                CandidAsset::new_with_content(random_10kb()),
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
                CandidAsset::new_with_content(random_100kb()),
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
                CandidAsset::new_with_content(random_1mb()),
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
                CandidAsset::new_with_content(random_2mb()),
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
                CborAsset::new_with_content(zero_bytes()),
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
                CborAsset::new_with_content(random_1kb()),
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
                CborAsset::new_with_content(random_10kb()),
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
                CborAsset::new_with_content(random_100kb()),
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
                CborAsset::new_with_content(random_1mb()),
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
                CborAsset::new_with_content(random_2mb()),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }
}
