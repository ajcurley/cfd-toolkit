/// Poylgonal face in a surface mesh
#[derive(Debug, Clone)]
pub struct Face {
    vertices: Vec<usize>,
    patch: Option<usize>,
}

impl Face {
    /// Construct a face from its vertices and patch
    pub fn new(vertices: Vec<usize>, patch: Option<usize>) -> Face {
        Face { vertices, patch }
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
