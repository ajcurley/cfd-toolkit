use pyo3::exceptions::PyIndexError;
use pyo3::prelude::*;

use crate::geometry::{Aabb, Vector};

/// Three-dimensional Triangle in Cartesian space
#[pyclass]
#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    p: Vector,
    q: Vector,
    r: Vector,
}

#[pymethods]
impl Triangle {
    /// Construct a Triangle from its vertices
    #[new]
    pub fn new(p: Vector, q: Vector, r: Vector) -> Triangle {
        Triangle { p, q, r }
    }

    /// Compute the center
    pub fn center(&self) -> Vector {
        (self.p + self.q + self.r) / 3.
    }

    /// Compute the axis-aligned bounding box
    pub fn aabb(&self) -> Aabb {
        let mut min = Vector::zeros();
        let mut max = Vector::zeros();

        for i in 0..3 {
            min[i] = self.p[i].min(self.q[i]).min(self.r[i]);
            max[i] = self.p[i].max(self.q[i]).max(self.r[i]);
        }

        Aabb::from_bounds(min, max)
    }

    /// Compute the area
    pub fn area(&self) -> f64 {
        self.normal().mag() * 0.5
    }

    /// Compute the normal Vector
    pub fn normal(&self) -> Vector {
        let u = self.q - self.p;
        let v = self.r - self.p;
        Vector::cross(&u, &v)
    }

    /// Compute the unit normal Vector
    pub fn unit_normal(&self) -> Vector {
        self.normal().unit()
    }

    /// Get a vertex by index
    pub fn __getitem__(&self, index: usize) -> PyResult<Vector> {
        if index > 2 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        Ok(self[index])
    }

    /// Set a vertex by index
    pub fn __setitem__(&mut self, index: usize, value: Vector) -> PyResult<()> {
        if index > 2 {
            return Err(PyIndexError::new_err("index out of range"));
        }

        self[index] = value;
        Ok(())
    }
}

impl std::ops::Index<usize> for Triangle {
    type Output = Vector;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.p,
            1 => &self.q,
            2 => &self.r,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Triangle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.p,
            1 => &mut self.q,
            2 => &mut self.r,
            _ => panic!("index out of range"),
        }
    }
}
