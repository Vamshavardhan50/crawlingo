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

    def test_session_cookie_and_retry_controls(self):
        session = crawlingo.Session()
        self.assertIs(session.cookies({"one": "1", "two": "2"}), session)
        snapshot = session.get_cookies()
        self.assertEqual(snapshot, {"one": "1", "two": "2"})
        snapshot["one"] = "changed"
        self.assertEqual(session.get_cookies()["one"], "1")
        self.assertIs(session.delete_cookie("one"), session)
        self.assertEqual(session.get_cookies(), {"two": "2"})
        self.assertIs(session.clear_cookies(), session)
        self.assertEqual(session.get_cookies(), {})
        self.assertIs(session.retries(2).retry_delay(1).retry_backoff(2), session)
        for call, value in [(session.retries, -1), (session.retries, 1.5),
                            (session.retry_delay, 86401), (session.retry_delay, True),
                            (session.retry_backoff, float("nan")), (session.retry_backoff, 0.5),
                            (session.retry_backoff, 10 ** 1000)]:
            with self.subTest(value=value), self.assertRaises(ValueError):
                call(value)


if __name__ == "__main__":
    unittest.main()
