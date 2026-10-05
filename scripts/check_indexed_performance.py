#!/usr/bin/env python3
"""Validate catalog benchmark samples and apply a coarse noise-aware scaling gate."""
import csv
import json
import math
from pathlib import Path
import statistics
import sys


def summarize(rows):
    groups, seen = {}, set()
    for row in rows:
        key = int(row["rules"]), row["implementation"]
        sample = int(row["sample"])
        value = float(row["ns_per_candidate"])
        if (key not in {(14, "linear"), (14, "indexed"), (214, "linear"), (214, "indexed")}
                or sample < 0 or (key, sample) in seen or not math.isfinite(value) or value <= 0):
            raise ValueError("Invalid or duplicate catalog benchmark sample")
        seen.add((key, sample))
        groups.setdefault(key, []).append((sample, value, int(row["checksum"])))
    if len(groups) != 4:
        raise ValueError("Expected both implementations at both catalog sizes")
    result = {}
    for key, samples in groups.items():
        if len(samples) < 5 or {s[0] for s in samples} != set(range(len(samples))):
            raise ValueError("Expected complete repeated samples")
        checksums = {s[2] for s in samples}
        if len(checksums) != 1:
            raise ValueError("Unstable result checksum")
        values = [s[1] for s in samples]
        median = statistics.median(values)
        result[key] = {"samples": values, "median": median,
                       "mad": statistics.median(abs(x - median) for x in values),
                       "checksum": checksums.pop()}
    for size in (14, 214):
        if result[size, "linear"]["checksum"] != result[size, "indexed"]["checksum"]:
            raise ValueError("Linear/indexed results differ")
    return result


def scaling_limit(small, large):
    return small["median"] * 2 + 20 + 6 * (small["mad"] + large["mad"])


def main():
    source = Path(sys.argv[1])
    with source.open() as stream:
        values = summarize(csv.DictReader(stream))
    small, large = values[14, "indexed"], values[214, "indexed"]
    limit = scaling_limit(small, large)
    report = {"summaries": {f"{size}/{kind}": data for (size, kind), data in values.items()},
              "threshold": "small median * 2 + 20 ns + 6 * combined MAD",
              "limit_ns": limit, "regression": large["median"] > limit}
    source.with_suffix(".json").write_text(json.dumps(report, indent=2) + "\n")
    print(f"Indexed: 14 rules {small['median']:.3f}, 214 rules {large['median']:.3f} ns/candidate; limit {limit:.3f}")
    if report["regression"]:
        raise SystemExit("Indexed catalog scaling exceeded the noise-aware budget")


if __name__ == "__main__":
    main()
