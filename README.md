# Heap vs Stable Canister Memory Benchmarks

This repository contains benchmarks for the Heap and Stable memory of a Canister.

## Results

- [Heap vs Stable for Structs in BTreeMap](./src/struct/results.md)

## Prerequisites

Requires [canbench](https://docs.rs/canbench-rs) to be installed.

```bash
cargo install canbench
```

## Running the Benchmarks

```bash
cd <case of your choice>
canbench
# e.g.
# cd src/struct
# canbench
```

## Updating the Results

```bash
cd <case of your choice>
canbench --persist
# e.g.
# cd src/struct
# canbench --persist
```

If you want to print a markdown table of the results, you can run the following command (assuming you have `python3` installed):

```bash
./scripts/report_markdown.py <canbench_results path>
# e.g.
# ./scripts/report_markdown.py ./src/struct/canbench_results.yml
```

Additionally, if you want to update the results in the folder of your choice, you can run the following command:

```bash
./scripts/report_markdown.py <canbench_results path> <markdown file path>
# e.g.
# ./scripts/report_markdown.py ./src/struct/canbench_results.yml ./src/struct/results.md
```
