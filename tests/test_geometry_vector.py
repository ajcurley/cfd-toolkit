import unittest

from toussaint import Vector


class TestVector(unittest.TestCase):
    """Unit tests for the geometry Vector class"""

    def test_add(self):
        """Test adding two Vectors"""
        u = Vector(-1, 3, 8)
        v = Vector(9, -9, 0)
        w = u + v

        self.assertEqual(w[0], 8)
        self.assertEqual(w[1], -6)
        self.assertEqual(w[2], 8)
