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
  buckets = {"heap": {}, "stable_cbor": {}, "stable_candid": {}}
  for name, node in benches.items():
    instr = node.get("instructions")
    if instr is None:
      continue
    if name.startswith("heap_read_"):
      buckets["heap"][name[len("heap_read_"):]] = instr
    elif name.startswith("stable_cbor_read_"):
      buckets["stable_cbor"][name[len("stable_cbor_read_"):]] = instr
    elif name.startswith("stable_candid_read_"):
      buckets["stable_candid"][name[len("stable_candid_read_"):]] = instr
  return buckets


def fmt_int(n):
  return f"{n:,}"


def replace_results_section(readme_path: Path, table_md: str) -> None:
  text = readme_path.read_text()
  # Find the start of the Results section header
  header_match = re.search(r'(?m)^##\s+Results\s*$', text)
  if header_match:
    start_idx = header_match.end()
    # Find the next header start after Results
    next_header_match = re.search(r'(?m)^##\s+[^\n]+', text[start_idx:])
    end_idx = start_idx + next_header_match.start() if next_header_match else len(text)
    # Replace the content between start and end with our table
    before = text[:start_idx]
    after = text[end_idx:]
    # Ensure exactly one blank line after the header
    if not before.endswith("\n"):
      before += "\n"
    # Collapse any extra blank lines at the start of the replaced region
    new_text = before + "\n" + table_md + "\n" + after.lstrip("\n")
    readme_path.write_text(new_text)
  else:
    # No Results section: append one at the end
    if not text.endswith("\n"):
      text += "\n"
    new_text = text + "\n## Results\n\n" + table_md + "\n"
    readme_path.write_text(new_text)


def main():
  results_path = Path("canbench_results.yml")
  readme_path = Path(sys.argv[1]) if len(sys.argv) > 1 else None
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
  lines.append("| Size | Heap | Stable CBOR | Stable Candid | CBOR/Heap | Candid/Heap |")
  lines.append("|------|-----:|------------:|--------------:|----------:|------------:|")

  for sz in sizes:
    h = data["heap"].get(sz)
    c = data["stable_cbor"].get(sz)
    a = data["stable_candid"].get(sz)
    rc = (c / h) if (h and c) else None
    rca = (a / h) if (h and a) else None
    lines.append("| {} | {} | {} | {} | {} | {} |".format(
      human_size(sz),
      fmt_int(h) if h is not None else "-",
      fmt_int(c) if c is not None else "-",
      fmt_int(a) if a is not None else "-",
      f"{rc:.1f}x" if rc else "-",
      f"{rca:.1f}x" if rca else "-",
    ))

  lines.append("")
  lines.append("Source: [canbench_results.yml](./canbench_results.yml)")
  lines.append("")

  table_md = "\n".join(lines)
  table_md = f"Wasm instructions used for each asset size:\n\n{table_md}"

  # Write into README Results section
  if readme_path and readme_path.exists():
    replace_results_section(readme_path, table_md)

  # Also print to stdout for convenience
  print(table_md)


if __name__ == "__main__":
  main()


