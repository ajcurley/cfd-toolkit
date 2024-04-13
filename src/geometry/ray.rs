use pyo3::prelude::*;

use crate::geometry::Vector;

/// One-sided infinite vector in three-dimensional Cartesian space
#[pyclass]
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Vector,
    direction: Vector,
}

#[pymethods]
impl Ray {
    /// Construct a Ray from its origin and direction
    #[new]
    pub fn new(origin: Vector, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    /// Get the origin
    pub fn origin(&self) -> Vector {
        self.origin
    }

    /// Get the direction
    pub fn direction(&self) -> Vector {
        self.direction
    }
}
