use meshx::geometry::Aabb;
use meshx::mesh::half_edge::HeMesh;
use meshx::mesh::{Edge, Face, Patch, Vertex};
use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct SurfaceMesh {
    data: HeMesh,
}

#[pymethods]
impl SurfaceMesh {
    /// Construct a SurfaceMesh from an OBJ file
    #[staticmethod]
    pub fn from_obj(path: &str) -> PyResult<SurfaceMesh> {
        match HeMesh::from_obj(path) {
            Ok(data) => Ok(SurfaceMesh { data }),
            Err(error) => Err(PyIOError::new_err(error.to_string())),
        }
    }

    /// Export a SurfaceMesh to an OBJ file
    pub fn export_obj(&self, path: &str) -> PyResult<()> {
        match self.data.export_obj(path) {
            Ok(()) => Ok(()),
            Err(error) => Err(PyIOError::new_err(error.to_string())),
        }
    }

    /// Get the number of vertices
    pub fn n_vertices(&self) -> usize {
        self.data.n_vertices()
    }

    /// Get the vertices
    pub fn vertices(&self) -> Vec<Vertex> {
        (0..self.n_vertices()).map(|i| self.vertex(i)).collect()
    }

    /// Get a vertex by index
    pub fn vertex(&self, index: usize) -> Vertex {
        self.data.vertex(index).point().into()
    }

    /// Get the number of faces
    pub fn n_faces(&self) -> usize {
        self.data.n_faces()
    }

    /// Get the faces
    pub fn faces(&self) -> Vec<Face> {
        (0..self.n_faces()).map(|i| self.face(i)).collect()
    }

    /// Get a face by index
    pub fn face(&self, index: usize) -> Face {
        let vertices = self.data.face_vertices(index);
        let patch = self.data.face(index).patch();
        Face::new(vertices, patch)
    }

    /// Get the number of edges. This is the number of half edges which
    /// counts each face's edges independently such that for each closed
    /// face-face pair, there are two edges.
    pub fn n_edges(&self) -> usize {
        self.data.n_half_edges()
    }

    /// Get the edges
    pub fn edges(&self) -> Vec<Edge> {
        (0..self.n_edges()).map(|i| self.edge(i)).collect()
    }

    /// Get an edge by index
    pub fn edge(&self, index: usize) -> Edge {
        let half_edge = self.data.half_edge(index);
        let next = self.data.half_edge(half_edge.next());
        let face = self.data.face(half_edge.face());
        let p = half_edge.origin();
        let q = next.origin();
        let patch = face.patch();
        Edge::new(p, q, patch)
    }

    /// Get the number of patches
    pub fn n_patches(&self) -> usize {
        self.data.n_patches()
    }

    /// Get the patches
    pub fn patches(&self) -> Vec<Patch> {
        self.data
            .patches()
            .iter()
            .map(|p| p.name().to_string())
            .map(|n| Patch::new(n))
            .collect()
    }

    /// Get a patch by index
    pub fn patch(&self, index: usize) -> Patch {
        let name = self.data.patch(index).name().to_string();
        Patch::new(name)
    }

    /// Compute the axis-aligned bounding box
    pub fn aabb(&self) -> Aabb {
        self.data.aabb()
    }

    /// Check if the surface mesh is closed
    pub fn is_closed(&self) -> bool {
        self.data.is_closed()
    }

    /// Check if the surface mesh is consistently oriented
    pub fn is_consistent(&self) -> bool {
        self.data.is_consistent()
    }

    /// Orient the surface mesh such that all adjacent faces share the same
    /// surface normal direction. Unconnected components may have opposing
    /// orientations.
    pub fn orient(&mut self) {
        self.data.orient();
    }

    /// Merge the surface mesh naively. This strictly adds the contents
    /// of the two meshes but does not merge any duplicates.
    pub fn merge(&mut self, other: &SurfaceMesh) {
        self.data.merge(&other.data);
    }

    /// Extract a subset of the surface mesh by the faces.
    pub fn extract_faces(&self, face_ids: Vec<usize>) -> SurfaceMesh {
        SurfaceMesh {
            data: self.data.extract_faces(&face_ids),
        }
    }

    /// Extract a subset of the surface mesh by the patches.
    pub fn extract_patches(&self, patches: Vec<String>) -> SurfaceMesh {
        SurfaceMesh {
            data: self.data.extract_patches(&patches),
        }
    }

    /// Compute the connected components of the surface mesh. Each component
    /// is defined by its faces.
    pub fn components(&self) -> Vec<Vec<usize>> {
        self.data.components()
    }

    /// Compute the feature edges. Each edge represents one-side of a pair
    /// of edges belonging the two to adjacent faces.
    pub fn feature_edges(&self, angle: f64) -> Vec<(usize, usize)> {
        self.data.feature_edges(angle)
    }
}
