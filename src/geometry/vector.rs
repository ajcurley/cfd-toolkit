use std::ops::{Index, IndexMut};
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

    /// Get a component by index
    fn __getitem__(&self, index: usize) -> f64 {
        self[index]
    }

    /// Set a component by index
    fn __setitem__(&mut self, index: usize, value: f64) {
        self[index] = value
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
