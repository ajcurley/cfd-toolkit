use pyo3::prelude::*;

use crate::geometry::Vector;

/// Axis-aligned bounding box
#[pyclass]
#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    center: Vector,
    halfsize: Vector,
}

#[pymethods]
impl Aabb {
    /// Construct an Aabb from its center and halfsize
    #[new]
    pub fn new(center: Vector, halfsize: Vector) -> Aabb {
        Aabb { center, halfsize }
    }

    /// Construct an Aabb from its bounds
    #[staticmethod]
    pub fn from_bounds(min: Vector, max: Vector) -> Aabb {
        let center = (min + max) * 0.5;
        let halfsize = (max - min) * 0.5;
        Aabb::new(center, halfsize)
    }

    /// Return the center
    pub fn center(&self) -> Vector {
        self.center
    }

    /// Return the halfsize
    pub fn halfsize(&self) -> Vector {
        self.halfsize
    }

    /// Return the min bound
    pub fn min(&self) -> Vector {
        self.center - self.halfsize
    }

    /// Return the max bound
    pub fn max(&self) -> Vector {
        self.center + self.halfsize
    }
}
