import math
import unittest

import pytest

from cfd_toolkit.geometry import Vector3


class TestVector3(unittest.TestCase):
    """Unit tests for the geometry Vector3 class"""

    def test_index(self):
        """Test accessing a component by index"""
        u = Vector3(3, 2, 1)
        self.assertEqual(u[0], 3)
        self.assertEqual(u[1], 2)
        self.assertEqual(u[2], 1)

    def test_index_out_of_range(self):
        """Test accessing a component by index out of range"""
        with pytest.raises(IndexError):
            u = Vector3(3, 2, 1)
            u[3]

    def test_add(self):
        """Test adding two Vector3s"""
        u = Vector3(-1, 3, 8)
        v = Vector3(9, -9, 0)
        w = u + v
        self.assertEqual(w, Vector3(8, -6, 8))

    def test_add_assign(self):
        """Test adding a Vector3 in place"""
        u = Vector3(-1, 3, 8)
        v = Vector3(9, -9, 0)
        u += v
        self.assertEqual(u, Vector3(8, -6, 8))

    def test_subtract(self):
        """Test subtracting two Vector3s"""
        u = Vector3(-1, 3, 8)
        v = Vector3(9, -9, 0)
        w = u - v
        self.assertEqual(w, Vector3(-10, 12, 8))

    def test_subtract_assign(self):
        """Test subtracing a Vector3 in place"""
        u = Vector3(-1, 3, 8)
        v = Vector3(9, -9, 0)
        u -= v
        self.assertEqual(u, Vector3(-10, 12, 8))

    def test_multiply(self):
        """Test multiplying two Vector3s"""
        u = Vector3(-1, 3, 8)
        v = Vector3(9, -9, 4)
        w = u * v
        self.assertEqual(w, Vector3(-9, -27, 32))

    def test_multiply_assign(self):
        """Test multiplying a Vector3 in place"""
        u = Vector3(-1, 3, 8)
        v = Vector3(9, -9, 4)
        u *= v
        self.assertEqual(u, Vector3(-9, -27, 32))

    def test_divide(self):
        """Test dividing two Vector3s"""
        u = Vector3(-1, 3, 8)
        v = Vector3(9, -9, 4)
        w = u / v
        self.assertEqual(w, Vector3(-1 / 9, -1 / 3, 2))

    def test_divide_assign(self):
        """Test dividing a Vector3 in place"""
        u = Vector3(-1, 3, 8)
        v = Vector3(9, -9, 4)
        u /= v
        self.assertEqual(u, Vector3(-1 / 9, -1 / 3, 2))

    def test_negative(self):
        """Test the negative of a Vector3"""

    def test_dot_product(self):
        """Test computing the dot product"""
        u = Vector3(3, 2, 1)
        v = Vector3(1, 0, -5)
        d = Vector3.dot(u, v)
        self.assertEqual(d, -2)

    def test_cross_product(self):
        """Test computing the cross product"""
        u = Vector3(3, 2, 1)
        v = Vector3(1, 0, -5)
        c = Vector3.cross(u, v)
        self.assertEqual(c, Vector3(-10, 16, -2))

    def test_magnitude(self):
        """Test computing the Vector3 magnitude"""
        m = Vector3(2, 3, 6).mag()
        self.assertEqual(m, 7)

    def test_unit(self):
        """Test computing the unit Vector3"""
        u = Vector3(2, 3, 6).unit()
        self.assertEqual(u[0], 2 / 7)
        self.assertEqual(u[1], 3 / 7)
        self.assertEqual(u[2], 6 / 7)

    def test_unit_zero_magnitude(self):
        """Test computing the unit Vector3 with a zero magnitude"""
        u = Vector3(0, 0, 0).unit()
        self.assertTrue(math.isnan(u[0]))
        self.assertTrue(math.isnan(u[1]))
        self.assertTrue(math.isnan(u[2]))
