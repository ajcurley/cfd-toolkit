use crate::geometry::Vector;

/// IOPoint in three-dimensional Cartesian space
#[derive(Debug, Copy, Clone)]
pub struct IOPoint {
    origin: Vector,
}

impl IOPoint {
    /// Construct a IOPoint from its components
    pub fn new(x: f64, y: f64, z: f64) -> IOPoint {
        IOPoint {
            origin: Vector::new(x, y, z),
        }
    }

    /// Get the origin
    pub fn origin(&self) -> Vector {
        self.origin
    }
}

/// Poylgonal face in a surface mesh
#[derive(Debug, Clone)]
pub struct IOFace {
    vertices: Vec<usize>,
    patch: Option<usize>,
}

impl IOFace {
    /// Construct a face from its vertices and patch
    pub fn new(vertices: Vec<usize>, patch: Option<usize>) -> IOFace {
        IOFace { vertices, patch }
    }

    /// Get a borrowed reference to the vertices
    pub fn vertices(&self) -> &Vec<usize> {
        &self.vertices
    }

    /// Get the patch
    pub fn patch(&self) -> Option<usize> {
        self.patch
    }
}

/// Patch for a group of faces
pub struct IOPatch {
    name: String,
}

impl IOPatch {
    /// Construct a Patch from its name
    pub fn new(name: &str) -> IOPatch {
        IOPatch {
            name: name.to_string(),
        }
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }
}
