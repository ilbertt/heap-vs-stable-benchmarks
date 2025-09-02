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

    use crate::{heap::with_heap_state, stable::with_stable_state, store::Store};

    #[bench(raw)]
    fn heap_read_0b() -> BenchResult {
        with_heap_state(|s| {
            s.set(zero_bytes());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn stable_read_0b() -> BenchResult {
        with_stable_state(|s| {
            s.set(zero_bytes());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn heap_read_1kb() -> BenchResult {
        with_heap_state(|s| {
            s.set(random_1kb());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn stable_read_1kb() -> BenchResult {
        with_stable_state(|s| {
            s.set(random_1kb());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn heap_read_10kb() -> BenchResult {
        with_heap_state(|s| {
            s.set(random_10kb());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn stable_read_10kb() -> BenchResult {
        with_stable_state(|s| {
            s.set(random_10kb());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn heap_read_100kb() -> BenchResult {
        with_heap_state(|s| {
            s.set(random_100kb());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn stable_read_100kb() -> BenchResult {
        with_stable_state(|s| {
            s.set(random_100kb());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn heap_read_1mb() -> BenchResult {
        with_heap_state(|s| {
            s.set(random_1mb());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn stable_read_1mb() -> BenchResult {
        with_stable_state(|s| {
            s.set(random_1mb());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn heap_read_2mb() -> BenchResult {
        with_heap_state(|s| {
            s.set(random_2mb());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }

    #[bench(raw)]
    fn stable_read_2mb() -> BenchResult {
        with_stable_state(|s| {
            s.set(random_2mb());

            canbench_rs::bench_fn(|| {
                black_box(s.get());
            })
        })
    }
}
