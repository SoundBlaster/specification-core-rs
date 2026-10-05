#!/usr/bin/env python3
"""Validate keyed static decision benchmark parity and coarse scaling."""

import csv
import json
import math
from pathlib import Path
import statistics
import sys

EXPECTED_GROUPS = {
    (14, "static"),
    (14, "handwritten"),
    (214, "static"),
    (214, "handwritten"),
}


def summarize(rows):
    groups = {}
    seen = set()
    for row in rows:
        try:
            rules = int(row["catalog"])
            implementation = row["implementation"]
            sample = int(row["round"])
            value = float(row["ns_per_candidate"])
            checksum = int(row["checksum"])
        except (KeyError, TypeError, ValueError) as error:
            raise ValueError("Malformed keyed static benchmark sample") from error

        key = (rules, implementation)
        if (key not in EXPECTED_GROUPS or sample < 0 or checksum < 0
                or not math.isfinite(value) or value <= 0):
            raise ValueError("Invalid keyed static benchmark sample")
        sample_key = (key, sample)
        if sample_key in seen:
            raise ValueError("Duplicate keyed static benchmark sample")
        seen.add(sample_key)
        groups.setdefault(key, []).append((sample, value, checksum))

    if set(groups) != EXPECTED_GROUPS:
        raise ValueError("Expected static and handwritten samples for 14 and 214 rules")

    result = {}
    shared_sample_ids = None
    for key, samples in groups.items():
        sample_ids = {sample for sample, _, _ in samples}
        if len(samples) < 5 or sample_ids != set(range(len(samples))):
            raise ValueError("Expected complete repeated benchmark rounds")
        if shared_sample_ids is None:
            shared_sample_ids = sample_ids
        elif sample_ids != shared_sample_ids:
            raise ValueError("Expected identical benchmark rounds in every group")
        checksums = {checksum for _, _, checksum in samples}
        if len(checksums) != 1:
            raise ValueError("Unstable benchmark checksum")
        values = [value for _, value, _ in samples]
        median = statistics.median(values)
        result[key] = {
            "samples_ns_per_candidate": values,
            "median_ns_per_candidate": median,
            "mad_ns_per_candidate": statistics.median(abs(value - median) for value in values),
            "checksum": checksums.pop(),
        }

    for rules in (14, 214):
        if result[rules, "static"]["checksum"] != result[rules, "handwritten"]["checksum"]:
            raise ValueError(f"Static/handwritten results differ for {rules} rules")
    if result[14, "static"]["checksum"] != result[214, "static"]["checksum"]:
        raise ValueError("14-rule/214-rule static results differ")
    return result


def scaling_limit(small, large):
    """Allow a 2x base plus 20 ns and six combined MADs for runner noise."""
    return (2 * small["median_ns_per_candidate"] + 20
            + 6 * (small["mad_ns_per_candidate"] + large["mad_ns_per_candidate"]))


def comparison_limit(reference, implementation):
    """Allow up to 8x handwritten time plus a coarse noise allowance."""
    return (8 * reference["median_ns_per_candidate"] + 20
            + 6 * (reference["mad_ns_per_candidate"]
                  + implementation["mad_ns_per_candidate"]))


def main():
    source = Path(sys.argv[1])
    with source.open(newline="") as stream:
        summaries = summarize(csv.DictReader(stream))

    small, large = summaries[14, "static"], summaries[214, "static"]
    limit = scaling_limit(small, large)
    comparison_limits = {
        str(rules): comparison_limit(summaries[rules, "handwritten"], summaries[rules, "static"])
        for rules in (14, 214)
    }
    comparison_regressions = [
        rules for rules in (14, 214)
        if summaries[rules, "static"]["median_ns_per_candidate"] > comparison_limits[str(rules)]
    ]
    scaling_regression = large["median_ns_per_candidate"] > limit
    report = {
        "summaries": {f"{rules}/{implementation}": summary
                      for (rules, implementation), summary in sorted(summaries.items())},
        "thresholds": {
            "scaling": "14-rule static median * 2 + 20 ns + 6 * combined MAD",
            "handwritten_comparison": "handwritten median * 8 + 20 ns + 6 * combined MAD",
        },
        "scaling_limit_ns_per_candidate": limit,
        "handwritten_comparison_limits_ns_per_candidate": comparison_limits,
        "scaling_regression": scaling_regression,
        "handwritten_comparison_regressions": comparison_regressions,
        "regression": scaling_regression or bool(comparison_regressions),
    }
    source.with_suffix(".json").write_text(json.dumps(report, indent=2) + "\n")
    print(
        "Keyed static: 14 rules "
        f"{small['median_ns_per_candidate']:.3f}, 214 rules "
        f"{large['median_ns_per_candidate']:.3f} ns/candidate; "
        f"limit {limit:.3f}"
    )
    for rules in (14, 214):
        static = summaries[rules, "static"]["median_ns_per_candidate"]
        handwritten = summaries[rules, "handwritten"]["median_ns_per_candidate"]
        print(
            f"{rules} rules: static {static:.3f}, handwritten {handwritten:.3f} "
            f"ns/candidate; comparison limit {comparison_limits[str(rules)]:.3f}"
        )
    if report["regression"]:
        raise SystemExit("Keyed static performance exceeded a noise-aware budget")


if __name__ == "__main__":
    main()
