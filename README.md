# Heap vs Stable Canister Memory Benchmarks

This repository contains benchmarks for the Heap and Stable memory of a Canister.

## Results

Wasm instructions used for each asset size:

| Size | Heap | Stable CBOR | Stable Candid | CBOR/Heap | Candid/Heap |
|------|-----:|------------:|--------------:|----------:|------------:|
| 0 B | 2,835 | 10,575 | 71,776 | 3.7x | 25.3x |
| 1 KB | 4,472 | 111,190 | 507,386 | 24.9x | 113.5x |
| 10 KB | 13,688 | 1,009,436 | 4,420,785 | 73.7x | 323.0x |
| 100 KB | 106,322 | 9,988,010 | 43,535,582 | 93.9x | 409.5x |
| 1 MB | 1,052,498 | 102,158,558 | 445,105,778 | 97.1x | 422.9x |
| 2 MB | 2,101,128 | 204,303,580 | 891,184,137 | 97.2x | 424.1x |

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
