# Heap vs Stable Canister Memory Benchmarks

This repository contains benchmarks for the Heap and Stable memory of a Canister.

## Results

- Heap vs Stable for Bytes ([src/bytes](./src/bytes/)):
  - [Results](./src/bytes/results.md)
- Heap vs Stable for Structs in BTreeMap ([src/struct](./src/struct/)):
  - [Results](./src/struct/results.md)

## Prerequisites

Requires [canbench](https://docs.rs/canbench-rs) to be installed.

```bash
cargo install canbench
```

## Running the Benchmarks

Navigate to the benchmarks case of your choice and run the benchmarks. For example:

```bash
cd src/struct
canbench
```

## Updating the Results

If you want to persist the results to the `canbench_results.yml` file in the benchmarks case of your choice, you can run the following command. For example:

```bash
cd src/struct
canbench --persist
```

If you want to print a markdown table of the results, you can run the following command (assuming you have `python3` installed):

```bash
cd src/struct
./scripts/report_markdown.py
```

Additionally, if you want to update the `results.md` file in the benchmarks case of your choice, you can run the following command:

```bash
cd src/struct
./scripts/report_markdown.py results.md
```
