use pyo3::prelude::*;

use crate::geometry::{Aabb, Vector};

/// Sphere in Cartesian coordinate space
#[pyclass]
#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    center: Vector,
    radius: Vector,
}

#[pymethods]
impl Sphere {
    /// Construct a Sphere from its center and radius
    #[new]
    pub fn new(center: Vector, radius: Vector) -> Sphere {
        Sphere { center, radius }
    }

    /// Get the center
    pub fn center(&self) -> Vector {
        self.center
    }

    /// Get the radius
    pub fn radius(&self) -> Vector {
        self.radius
    }

    /// Compute the axis-aligned bounding box
    pub fn aabb(&self) -> Aabb {
        Aabb::new(self.center, self.radius)
    }
}
