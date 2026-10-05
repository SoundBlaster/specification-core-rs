import unittest

from check_static_performance import comparison_limit, scaling_limit, summarize


def rows():
    return [
        {
            "catalog": str(size),
            "implementation": implementation,
            "round": str(sample),
            "ns_per_candidate": str(rate),
            "checksum": "987654",
        }
        for size, implementation, rate in (
            (14, "static", 50),
            (14, "handwritten", 40),
            (214, "static", 54),
            (214, "handwritten", 42),
        )
        for sample in range(6)
    ]


class StaticPerformanceReportTests(unittest.TestCase):
    def test_samples_parity_and_scaling_budget(self):
        summaries = summarize(rows())
        self.assertLess(
            summaries[214, "static"]["median_ns_per_candidate"],
            scaling_limit(summaries[14, "static"], summaries[214, "static"]),
        )
        # The gate is a scaling check; it does not require beating handwritten code.
        self.assertGreater(
            summaries[14, "static"]["median_ns_per_candidate"],
            summaries[14, "handwritten"]["median_ns_per_candidate"],
        )

    def test_missing_group_and_incomplete_samples_rejected(self):
        missing_group = [
            row for row in rows()
            if not (row["catalog"] == "214" and row["implementation"] == "handwritten")
        ]
        incomplete_rounds = rows()
        incomplete_rounds[:] = [
            row for row in incomplete_rounds
            if not (row["catalog"] == "14" and row["implementation"] == "static"
                    and row["round"] == "2")
        ]
        for invalid in (missing_group, incomplete_rounds):
            with self.subTest(samples=len(invalid)), self.assertRaises(ValueError):
                summarize(invalid)

    def test_groups_require_identical_round_ids(self):
        invalid = [
            row for row in rows()
            if not (row["catalog"] == "14" and row["implementation"] == "static"
                    and row["round"] == "5")
        ]
        with self.assertRaisesRegex(ValueError, "identical benchmark rounds"):
            summarize(invalid)

    def test_duplicate_samples_rejected(self):
        invalid = rows()
        invalid.append(dict(invalid[0]))
        with self.assertRaisesRegex(ValueError, "Duplicate"):
            summarize(invalid)

    def test_handwritten_checksum_mismatch_rejected(self):
        invalid = rows()
        for row in invalid:
            if row["catalog"] == "214" and row["implementation"] == "handwritten":
                row["checksum"] = "123"
        with self.assertRaisesRegex(ValueError, "differ"):
            summarize(invalid)

    def test_unstable_checksum_rejected(self):
        invalid = rows()
        invalid[0]["checksum"] = "123"
        with self.assertRaisesRegex(ValueError, "Unstable"):
            summarize(invalid)

    def test_nonfinite_and_nonpositive_times_rejected(self):
        for value in ("nan", "inf", "0", "-1"):
            invalid = rows()
            invalid[0]["ns_per_candidate"] = value
            with self.subTest(value=value), self.assertRaises(ValueError):
                summarize(invalid)

    def test_large_handwritten_regression_exceeds_comparison_budget(self):
        summaries = summarize(rows())
        reference = summaries[214, "handwritten"]
        implementation = summaries[214, "static"]
        self.assertLess(
            implementation["median_ns_per_candidate"],
            comparison_limit(reference, implementation),
        )

        regressed = dict(implementation)
        regressed["median_ns_per_candidate"] = 400
        self.assertGreater(
            regressed["median_ns_per_candidate"],
            comparison_limit(reference, regressed),
        )


if __name__ == "__main__":
    unittest.main()
