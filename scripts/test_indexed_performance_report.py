import unittest

from check_indexed_performance import scaling_limit, summarize


def rows():
    return [{"rules": str(size), "implementation": kind, "sample": str(i),
             "ns_per_candidate": str(rate), "checksum": str(size)}
            for size, kind, rate in [(14, "linear", 50), (14, "indexed", 20),
                                     (214, "linear", 370), (214, "indexed", 24)] for i in range(5)]


class IndexedReportTests(unittest.TestCase):
    def test_samples_and_gate(self):
        values = summarize(rows())
        self.assertLess(values[214, "indexed"]["median"],
                        scaling_limit(values[14, "indexed"], values[214, "indexed"]))

    def test_missing_group_and_duplicate_rejected(self):
        for invalid in [rows()[:-5], rows() + [rows()[0]]]:
            with self.assertRaises(ValueError):
                summarize(invalid)

    def test_checksum_mismatch_rejected(self):
        invalid = rows()
        for row in invalid:
            if row["implementation"] == "indexed":
                row["checksum"] = "0"
        with self.assertRaises(ValueError):
            summarize(invalid)

    def test_invalid_time_rejected(self):
        for value in ["nan", "inf", "0", "-1"]:
            invalid = rows()
            invalid[0]["ns_per_candidate"] = value
            with self.assertRaises(ValueError):
                summarize(invalid)


if __name__ == "__main__":
    unittest.main()
