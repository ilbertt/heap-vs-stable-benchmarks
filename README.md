# Heap vs Stable Canister Memory Benchmarks

This repository contains benchmarks for the Heap and Stable memory of a Canister.

## Results

Wasm instructions used for each asset size:

| Size | Heap | Stable CBOR | Stable Candid | CBOR/Heap | Candid/Heap |
|------|-----:|------------:|--------------:|----------:|------------:|
| 0 B | 2,830 | 10,562 | 72,629 | 3.7x | 25.7x |
| 1 KB | 4,467 | 14,020 | 76,161 | 3.1x | 17.0x |
| 10 KB | 13,683 | 36,746 | 100,408 | 2.7x | 7.3x |
| 100 KB | 106,317 | 260,120 | 323,685 | 2.4x | 3.0x |
| 1 MB | 1,052,493 | 2,543,948 | 2,607,609 | 2.4x | 2.5x |
| 2 MB | 2,101,123 | 5,073,996 | 5,137,730 | 2.4x | 2.4x |

Source: [canbench_results.yml](./canbench_results.yml)

## Prerequisites

Requires [canbench](https://docs.rs/canbench-rs) to be installed.

```bash
cargo install canbench
```

## Running the Benchmarks

```bash
canbench
```

## Updating the Results

```bash
canbench --persist
```

If you want to print a markdown table of the results, you can run the following command (assuming you have `python3` installed):

```bash
./scripts/report_markdown.py
```

Additionally, if you want to update the results in the `README.md` file, you can run the following command:

```bash
./scripts/report_markdown.py README.md
```
