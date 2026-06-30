import unittest
from importlib.metadata import version

import crawlingo


class ImportSmokeTest(unittest.TestCase):
    def test_package_exports_match_metadata(self):
        self.assertEqual(crawlingo.__version__, version("crawlingo"))
        self.assertTrue(callable(crawlingo.Page))
        self.assertTrue(callable(crawlingo.Session))
        self.assertTrue(callable(crawlingo.Dataset))
        self.assertTrue(callable(crawlingo.Crawl))
        self.assertTrue(callable(crawlingo.Watch))


if __name__ == "__main__":
    unittest.main()
