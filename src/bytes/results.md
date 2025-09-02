Wasm instructions used for each bytes array size:

| Size | Heap | Stable | Stable/Heap |
|------|-----:|------:|------------:|
| 0 B | 694 | 855 | 1.2x |
| 1 KB | 1,696 | 2,880 | 1.7x |
| 10 KB | 10,912 | 21,312 | 2.0x |
| 100 KB | 103,546 | 205,632 | 2.0x |
| 1 MB | 1,049,722 | 2,097,984 | 2.0x |
| 2 MB | 2,098,352 | 4,195,136 | 2.0x |

Source: [canbench_results.yml](./canbench_results.yml)
