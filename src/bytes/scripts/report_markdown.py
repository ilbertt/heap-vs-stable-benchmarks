#!/usr/bin/env python3

import re
import sys
from pathlib import Path

SIZES_ORDER = ["0b", "1kb", "10kb", "100kb", "1mb", "2mb"]


def parse_yaml(text: str):
  benches = {}
  in_benches = False
  current = None
  for line in text.splitlines():
    if not in_benches:
      if re.match(r'^\s*benches:\s*$', line):
        in_benches = True
      continue
    if re.match(r'^[^\s]', line):
      break
    m_bench = re.match(r'^\s{2}([^\s][^:]*):\s*$', line)
    if m_bench:
      current = m_bench.group(1).strip()
      benches[current] = {}
      continue
    if current:
      m_instr = re.search(r'^\s+instructions:\s*(\d+)\s*$', line)
      if m_instr:
        benches[current]["instructions"] = int(m_instr.group(1))
  return {"benches": benches}


def size_to_bytes(size: str) -> int:
  m = re.match(r'^(\d+)(b|kb|mb)$', size)
  if not m:
    return 0
  n = int(m.group(1))
  unit = m.group(2)
  return n if unit == "b" else n * 1024 if unit == "kb" else n * 1024 * 1024


def human_size(size: str) -> str:
  if size.endswith("mb"):
    return f"{int(size[:-2])} MB"
  if size.endswith("kb"):
    return f"{int(size[:-2])} KB"
  if size.endswith("b"):
    return f"{int(size[:-1])} B"
  return size


def collect(results):
  benches = results.get("benches", {})
  buckets = {"heap": {}, "stable": {}}
  for name, node in benches.items():
    instr = node.get("instructions")
    if instr is None:
      continue
    if name.startswith("heap_read_"):
      buckets["heap"][name[len("heap_read_"):]] = instr
    elif name.startswith("stable_read_"):
      buckets["stable"][name[len("stable_read_"):]] = instr
  return buckets


def fmt_int(n):
  return f"{n:,}"


def main():
  results_path = Path("./canbench_results.yml")
  output_path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("./results.md")
  if not results_path.exists():
    print(f"File not found: {results_path}", file=sys.stderr)
    sys.exit(1)

  results_text = results_path.read_text()
  results = parse_yaml(results_text)
  data = collect(results)

  sizes = sorted(
    set().union(*[set(d.keys()) for d in data.values()]) if data else set(),
    key=lambda s: (size_to_bytes(s), SIZES_ORDER.index(s) if s in SIZES_ORDER else 10**9),
  )

  lines = []
  lines.append("Wasm instructions used for each bytes array size:\n")
  lines.append("| Size | Heap | Stable | Stable/Heap |")
  lines.append("|------|-----:|------:|------------:|")

  for sz in sizes:
    h = data["heap"].get(sz)
    s = data["stable"].get(sz)
    rs = (s / h) if (h and s) else None
    lines.append("| {} | {} | {} | {} |".format(
      human_size(sz),
      fmt_int(h) if h is not None else "-",
      fmt_int(s) if s is not None else "-",
      f"{rs:.1f}x" if rs else "-",
    ))

  lines.append("")
  lines.append("Source: [canbench_results.yml](./canbench_results.yml)")
  lines.append("")

  table_md = "\n".join(lines)

  # Write into results.md (or provided path)
  output_path.write_text(table_md)

  # Also print to stdout for convenience
  print(table_md)


if __name__ == "__main__":
  main()
