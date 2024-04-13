use std::collections::BTreeMap;

use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;

/// Three-dimensional Cartesian vector
#[pyclass]
#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

#[pymethods]
impl Vector {
    /// Construct a new Vector from its components
    #[new]
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    /// Construct a new Vector of all zeros
    #[staticmethod]
    pub fn zeros() -> Vector {
        Vector::new(0., 0., 0.)
    }

    /// Construct a new Vector of all ones
    #[staticmethod]
    pub fn ones() -> Vector {
        Vector::new(1., 1., 1.)
    }

    /// Compute the Vector dot product u * v
    #[staticmethod]
    pub fn dot(u: &Vector, v: &Vector) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    /// Compute the Vector cross product u x v
    #[staticmethod]
    pub fn cross(u: &Vector, v: &Vector) -> Vector {
        Vector {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
    }

    /// Compute the unit Vector
    pub fn unit(&self) -> Vector {
        *self / self.mag()
    }

    /// Convert the Vector to a list
    pub fn list(&self) -> Vec<f64> {
        vec![self.x, self.y, self.z]
    }

    /// Convert the Vector to a dictionary (map)
    pub fn dict(&self) -> BTreeMap<&str, f64> {
        BTreeMap::from([("x", self.x), ("y", self.y), ("z", self.z)])
    }

    /// Get the string representation
    pub fn __str__(&self) -> String {
        format!("<Vector x={} y={} z={}>", self.x, self.y, self.z)
    }

    /// Get the internal string representation
    pub fn __repr__(&self) -> String {
        self.__str__()
    }

    /// Get a component by index
    pub fn __getitem__(&self, index: usize) -> PyResult<f64> {
        if index > 2 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        Ok(self[index])
    }

    /// Set a component by index
    pub fn __setitem__(&mut self, index: usize, value: f64) -> PyResult<()> {
        if index > 2 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        self[index] = value;
        Ok(())
    }

    /// Check the equality with another Vector
    pub fn __eq__(&self, other: Vector) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    /// Add a Vector using the + operator
    pub fn __add__(&self, other: Vector) -> Vector {
        *self + other
    }

    /// Add a Vector using the += operator
    pub fn __iadd__(&mut self, other: Vector) {
        *self += other;
    }

    /// Subtract a Vector using the - operator
    pub fn __sub__(&self, other: Vector) -> Vector {
        *self - other
    }

    /// Subtract a Vector using the -= operator
    pub fn __isub__(&mut self, other: Vector) {
        *self -= other;
    }

    /// Multiply a Vector using the * operator
    pub fn __mul__(&self, other: Vector) -> Vector {
        *self * other
    }

    /// Multiple a Vector using the *= operator
    pub fn __imul__(&mut self, other: Vector) {
        *self *= other
    }

    /// Divide a Vector using the / operator
    pub fn __truediv__(&self, other: Vector) -> Vector {
        *self / other
    }

    /// Divide a Vector using the /= operator
    pub fn __itruediv__(&mut self, other: Vector) {
        *self /= other
    }

    /// Compute the negative
    pub fn __neg__(&self) -> Vector {
        -(*self)
    }

    /// Compute the magnitude
    pub fn mag(&self) -> f64 {
        Vector::dot(self, self).sqrt()
    }
}

impl std::ops::Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Add<f64> for Vector {
    type Output = Vector;

    fn add(self, other: f64) -> Self::Output {
        Vector {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl std::ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::AddAssign<f64> for Vector {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Sub<f64> for Vector {
    type Output = Vector;

    fn sub(self, other: f64) -> Self::Output {
        Vector {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl std::ops::SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl std::ops::SubAssign<f64> for Vector {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl std::ops::Mul<Vector> for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Self::Output {
        Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl std::ops::MulAssign<Vector> for Vector {
    fn mul_assign(&mut self, other: Vector) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl std::ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl std::ops::Div<Vector> for Vector {
    type Output = Vector;

    fn div(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl std::ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Self::Output {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl std::ops::DivAssign<Vector> for Vector {
    fn div_assign(&mut self, other: Vector) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl std::ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
