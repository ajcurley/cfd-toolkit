import unittest

from cfd_toolkit.geometry import AABB, Vector


class TestAABB(unittest.TestCase):
    """Unit tests for the geometry AABB class"""

    def test_from_bounds(self):
        """Test constructing an AABB from its min/max bounds"""
        min_bound = Vector(0, 0, 0)
        max_bound = Vector(1, 1, 1)
        aabb = AABB.from_bounds(min_bound, max_bound)

        self.assertEqual(aabb.center(), Vector(0.5, 0.5, 0.5))
        self.assertEqual(aabb.halfsize(), Vector(0.5, 0.5, 0.5))

    def test_min_max(self):
        """Test computing the min/max bounds"""
        center = Vector(0, 0, 0)
        halfsize = Vector(1, 1, 1)
        aabb = AABB(center, halfsize)

        self.assertEqual(aabb.min(), Vector(-1, -1, -1))
        self.assertEqual(aabb.max(), Vector(1, 1, 1))
