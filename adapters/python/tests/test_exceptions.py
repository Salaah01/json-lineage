from unittest import TestCase

from json_lineage import exceptions


class BinaryExecutionException(TestCase):
    def test__str__returns_a_string(self):
        """Test that the `__str__` method returns a string."""
        err = exceptions.BinaryExecutionException(b"error")
        self.assertIsInstance(str(err), str)
