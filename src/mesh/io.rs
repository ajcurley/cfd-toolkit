use crate::geometry::Vector;

/// IOVertex in three-dimensional Cartesian space
#[derive(Debug, Copy, Clone, Default)]
pub struct IOVertex {
    origin: Vector,
}

impl IOVertex {
    /// Construct a IOVertex from its components
    pub fn new(x: f64, y: f64, z: f64) -> IOVertex {
        IOVertex {
            origin: Vector::new(x, y, z),
        }
    }

    /// Get the origin
    pub fn origin(&self) -> Vector {
        self.origin
    }
}

impl From<&Vec<f64>> for IOVertex {
    fn from(values: &Vec<f64>) -> IOVertex {
        assert_eq!(values.len(), 3);
        let x = values[0];
        let y = values[1];
        let z = values[2];
        IOVertex::new(x, y, z)
    }
}

/// Poylgonal face in a surface mesh
#[derive(Debug, Clone, Default)]
pub struct IOFace {
    vertices: Vec<usize>,
    patch: Option<usize>,
}

impl IOFace {
    /// Construct a face from its vertices
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
#[derive(Debug, Clone, Default)]
pub struct IOPatch {
    name: String,
}

impl IOPatch {
    /// Construct a Patch from its name
    pub fn new(name: String) -> IOPatch {
        IOPatch { name }
    }

    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Polygonal surface mesh for IO
#[derive(Debug, Clone, Default)]
pub struct IOMesh {
    vertices: Vec<IOVertex>,
    faces: Vec<IOFace>,
    patches: Vec<IOPatch>,
}

impl IOMesh {
    /// Insert a vertex
    pub fn insert_vertex(&mut self, vertex: IOVertex) -> usize {
        let index = self.vertices.len();
        self.vertices.push(vertex);
        index
    }

    /// Get the number of vertices
    pub fn number_of_vertices(&self) -> usize {
        self.vertices.len()
    }

    /// Insert a face
    pub fn insert_face(&mut self, face: IOFace) -> usize {
        let index = self.faces.len();
        self.faces.push(face);
        index
    }

    /// Get the number of faces
    pub fn number_of_faces(&self) -> usize {
        self.faces.len()
    }

    /// Insert a patch
    pub fn insert_patch(&mut self, patch: IOPatch) -> usize {
        let index = self.patches.len();
        self.patches.push(patch);
        index
    }

    /// Get the number of patches
    pub fn number_of_patches(&self) -> usize {
        self.patches.len()
    }

    /// Get the latest patch index
    pub fn latest_patch(&self) -> Option<usize> {
        if self.patches.is_empty() {
            return None;
        }

        Some(self.number_of_patches() - 1)
    }
}
