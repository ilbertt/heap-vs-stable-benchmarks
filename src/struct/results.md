Wasm instructions used for each asset size:

| Size | Heap | Stable CBOR | Stable Candid | CBOR/Heap | Candid/Heap |
|------|-----:|------------:|--------------:|----------:|------------:|
| 0 B | 2,835 | 10,562 | 72,577 | 3.7x | 25.6x |
| 1 KB | 4,472 | 14,032 | 76,121 | 3.1x | 17.0x |
| 10 KB | 13,688 | 36,791 | 100,401 | 2.7x | 7.3x |
| 100 KB | 106,322 | 260,447 | 323,960 | 2.4x | 3.0x |
| 1 MB | 1,052,498 | 2,547,089 | 2,610,698 | 2.4x | 2.5x |
| 2 MB | 2,101,128 | 5,080,245 | 5,143,927 | 2.4x | 2.4x |

Source: [canbench_results.yml](./canbench_results.yml)
