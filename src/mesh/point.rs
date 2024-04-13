use crate::geometry::Vector;

/// Point in three-dimensional Cartesian space
#[derive(Debug, Copy, Clone)]
pub struct Point {
    origin: Vector,
}

impl Point {
    /// Construct a Point from its components
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {
            origin: Vector::new(x, y, z),
        }
    }

    /// Get the origin
    pub fn origin(&self) -> Vector {
        self.origin
    }
}

impl std::ops::Index<usize> for Point {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.origin[index]
    }
}

impl std::ops::IndexMut<usize> for Point {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.origin[index]
    }
}
