"""Exercise Python wrapper behavior independently of a compiled extension."""
import importlib.util
import pathlib
import sys
import types
import unittest
from unittest.mock import Mock


def load_session():
    package = types.ModuleType("_crawlingo_session_test")
    package.__path__ = []
    native = types.ModuleType("_crawlingo_session_test._crawlingo_core")
    native.Session = Mock
    sys.modules[package.__name__] = package
    sys.modules[native.__name__] = native
    path = pathlib.Path(__file__).resolve().parents[1] / "crawlingo" / "session.py"
    spec = importlib.util.spec_from_file_location(package.__name__ + ".session", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module.Session


class SessionWrapperTest(unittest.TestCase):
    def test_controls_forward_arguments_and_preserve_chaining(self):
        session = load_session()()
        native = session._core_session
        native.get_cookies.return_value = {"locale": "en"}
        self.assertEqual(session.get_cookies(), {"locale": "en"})
        self.assertIs(session.delete_cookie("locale").clear_cookies(), session)
        self.assertIs(session.retries(2).retry_delay(1).retry_backoff(2.5), session)
        native.delete_cookie.assert_called_once_with("locale")
        native.clear_cookies.assert_called_once_with()
        native.retries.assert_called_once_with(2)
        native.retry_delay.assert_called_once_with(1)
        native.retry_backoff.assert_called_once_with(2.5)

    def test_invalid_retry_values_are_rejected_before_native_calls(self):
        session = load_session()()
        for method, invalid in [
            ("retries", [-1, 1.5, True, 2**32]),
            ("retry_delay", [-1, True, 86401, 0.5]),
            ("retry_backoff", [0.5, True, float("inf"), float("nan"), 10**1000]),
        ]:
            for value in invalid:
                with self.subTest(method=method, value=value), self.assertRaises(ValueError):
                    getattr(session, method)(value)
            getattr(session._core_session, method).assert_not_called()


if __name__ == "__main__":
    unittest.main()
