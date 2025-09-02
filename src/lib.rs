mod asset;
mod heap;
mod stable;
mod store;

#[cfg(feature = "canbench-rs")]
mod bench {
    use std::hint::black_box;

    use canbench_rs::{bench, BenchResult};

    use crate::{
        asset::Asset,
        heap::with_heap_state,
        stable::candid::{asset::CandidAsset, stable::with_candid_stable_state},
        stable::cbor::{asset::CborAsset, stable::with_cbor_stable_state},
        store::Store,
    };

    const ASSET_TEST_KEY: &str = "test";

    const ONE_KB: usize = 1024;
    const TEN_KB: usize = 10 * ONE_KB;
    const HUNDRED_KB: usize = 100 * ONE_KB;
    const ONE_MB: usize = ONE_KB * ONE_KB;
    const TWO_MB: usize = 2 * ONE_KB * ONE_KB;

    const ZERO_B_ASSET_CONTENT: &[u8] = &[0; 0];
    const ONE_KB_ASSET_CONTENT: &[u8] = &[0; ONE_KB];
    const TEN_KB_ASSET_CONTENT: &[u8] = &[0; TEN_KB];
    const HUNDRED_KB_ASSET_CONTENT: &[u8] = &[0; HUNDRED_KB];
    const ONE_MB_ASSET_CONTENT: &[u8] = &[0; ONE_MB];
    const TWO_MB_ASSET_CONTENT: &[u8] = &[0; TWO_MB];

    #[bench(raw)]
    fn heap_read_0b() -> BenchResult {
        with_heap_state(|s| {
            s.set(
                ASSET_TEST_KEY.to_string(),
                Asset::new_with_content(ZERO_B_ASSET_CONTENT.to_vec()),
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
                Asset::new_with_content(ONE_KB_ASSET_CONTENT.to_vec()),
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
                Asset::new_with_content(TEN_KB_ASSET_CONTENT.to_vec()),
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
                Asset::new_with_content(HUNDRED_KB_ASSET_CONTENT.to_vec()),
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
                Asset::new_with_content(ONE_MB_ASSET_CONTENT.to_vec()),
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
                Asset::new_with_content(TWO_MB_ASSET_CONTENT.to_vec()),
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
                CandidAsset::new_with_content(ZERO_B_ASSET_CONTENT.to_vec()),
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
                CandidAsset::new_with_content(ONE_KB_ASSET_CONTENT.to_vec()),
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
                CandidAsset::new_with_content(TEN_KB_ASSET_CONTENT.to_vec()),
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
                CandidAsset::new_with_content(HUNDRED_KB_ASSET_CONTENT.to_vec()),
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
                CandidAsset::new_with_content(ONE_MB_ASSET_CONTENT.to_vec()),
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
                CandidAsset::new_with_content(TWO_MB_ASSET_CONTENT.to_vec()),
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
                CborAsset::new_with_content(ZERO_B_ASSET_CONTENT.to_vec()),
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
                CborAsset::new_with_content(ONE_KB_ASSET_CONTENT.to_vec()),
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
                CborAsset::new_with_content(TEN_KB_ASSET_CONTENT.to_vec()),
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
                CborAsset::new_with_content(HUNDRED_KB_ASSET_CONTENT.to_vec()),
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
                CborAsset::new_with_content(ONE_MB_ASSET_CONTENT.to_vec()),
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
                CborAsset::new_with_content(TWO_MB_ASSET_CONTENT.to_vec()),
            );

            canbench_rs::bench_fn(|| {
                black_box(s.get(black_box(ASSET_TEST_KEY.to_string())));
            })
        })
    }
}
