import math
import unittest

import pytest

from cfd_toolkit.geometry import Vector


class TestVector(unittest.TestCase):
    """Unit tests for the geometry Vector class"""

    def test_index(self):
        """Test accessing a component by index"""
        u = Vector(3, 2, 1)
        self.assertEqual(u.list(), [3, 2, 1])

    def test_index_out_of_range(self):
        """Test accessing a component by index out of range"""
        with pytest.raises(IndexError):
            u = Vector(3, 2, 1)
            u[3]

    def test_add(self):
        """Test adding two Vectors"""
        u = Vector(-1, 3, 8)
        v = Vector(9, -9, 0)
        w = u + v
        self.assertEqual(w.list(), [8, -6, 8])

    def test_add_assign(self):
        """Test adding a Vector in place"""
        u = Vector(-1, 3, 8)
        v = Vector(9, -9, 0)
        u += v
        self.assertEqual(u.list(), [8, -6, 8])

    def test_subtract(self):
        """Test subtracting two Vectors"""
        u = Vector(-1, 3, 8)
        v = Vector(9, -9, 0)
        w = u - v
        self.assertEqual(w.list(), [-10, 12, 8])

    def test_subtract_assign(self):
        """Test subtracing a Vector in place"""
        u = Vector(-1, 3, 8)
        v = Vector(9, -9, 0)
        u -= v
        self.assertEqual(u.list(), [-10, 12, 8])

    def test_multiply(self):
        """Test multiplying two Vectors"""
        u = Vector(-1, 3, 8)
        v = Vector(9, -9, 4)
        w = u * v
        self.assertEqual(w.list(), [-9, -27, 32])

    def test_multiply_assign(self):
        """Test multiplying a Vector in place"""
        u = Vector(-1, 3, 8)
        v = Vector(9, -9, 4)
        u *= v
        self.assertEqual(u.list(), [-9, -27, 32])

    def test_divide(self):
        """Test dividing two Vectors"""
        u = Vector(-1, 3, 8)
        v = Vector(9, -9, 4)
        w = u / v
        self.assertEqual(w.list(), [-1 / 9, -1 / 3, 2])

    def test_divide_assign(self):
        """Test dividing a Vector in place"""
        u = Vector(-1, 3, 8)
        v = Vector(9, -9, 4)
        u /= v
        self.assertEqual(u.list(), [-1 / 9, -1 / 3, 2])

    def test_negative(self):
        """Test the negative of a Vector"""

    def test_dot_product(self):
        """Test computing the dot product"""
        u = Vector(3, 2, 1)
        v = Vector(1, 0, -5)
        d = Vector.dot(u, v)
        self.assertEqual(d, -2)

    def test_cross_product(self):
        """Test computing the cross product"""
        u = Vector(3, 2, 1)
        v = Vector(1, 0, -5)
        c = Vector.cross(u, v)
        self.assertEqual(c.list(), [-10, 16, -2])

    def test_magnitude(self):
        """Test computing the Vector magnitude"""
        m = Vector(2, 3, 6).mag()
        self.assertEqual(m, 7)

    def test_unit(self):
        """Test computing the unit Vector"""
        u = Vector(2, 3, 6).unit()
        self.assertEqual(u[0], 2 / 7)
        self.assertEqual(u[1], 3 / 7)
        self.assertEqual(u[2], 6 / 7)

    def test_unit_zero_magnitude(self):
        """Test computing the unit Vector with a zero magnitude"""
        u = Vector(0, 0, 0).unit()
        self.assertTrue(math.isnan(u[0]))
        self.assertTrue(math.isnan(u[1]))
        self.assertTrue(math.isnan(u[2]))

    def test_list(self):
        """Test converting to a Python list"""
        u = Vector(3, 2, 1).list()
        self.assertEqual(u, [3, 2, 1])

    def test_dict(self):
        """Test converting to a Python dict"""
        u = Vector(3, 2, 1).dict()
        self.assertEqual(u, {"x": 3, "y": 2, "z": 1})
