use std::ops::{Add, AddAssign, Index, IndexMut};

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

    /// Compute the Vector dot product u * v
    #[staticmethod]
    pub fn dot(u: Vector, v: Vector) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    /// Compute the Vector cross product u x v
    #[staticmethod]
    pub fn cross(u: Vector, v: Vector) -> Vector {
        Vector {
            x: u.y * v.z - u.z * v.y,
            y: u.z * v.x - u.x * v.z,
            z: u.x * v.y - u.y * v.x,
        }
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

    /// Add a Vector using the + operator
    pub fn __add__(&self, other: Vector) -> Vector {
        *self + other
    }

    /// Add a Vector using the += operator
    pub fn __iadd__(&mut self, other: Vector) {
        *self += other;
    }
}

impl Index<usize> for Vector {
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

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("index out of range"),
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<f64> for Vector {
    type Output = Vector;

    fn add(self, other: f64) -> Self::Output {
        Vector {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, other: Vector) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<f64> for Vector {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}
