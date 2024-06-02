use meshx::mesh::half_edge::HeMesh;
use meshx::mesh::Edge;
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

    /// Check if the surface mesh is closed
    pub fn is_closed(&self) -> bool {
        self.data.is_closed()
    }

    /// Check if the surface mesh is consistently oriented
    pub fn is_consistent(&self) -> bool {
        self.data.is_consistent()
    }

    /// Compute the feature edges. Each edge represents one-side of a pair
    /// of edges belonging the two to adjacent faces.
    pub fn feature_edges(&self, angle: f64) -> Vec<(Edge, Edge)> {
        let mut edges = vec![];

        for (i, j) in self.data.feature_edges(angle) {
            let half_edge = self.data.half_edge(i);
            let next = half_edge.next();
            let p = half_edge.origin();
            let q = self.data.half_edge(next).origin();

            let face = self.data.face(half_edge.face());
            let edge_i = Edge::new(p, q, face.patch());

            let half_edge = self.data.half_edge(j);
            let face = self.data.face(half_edge.face());
            let edge_j = Edge::new(p, q, face.patch());

            edges.push((edge_i, edge_j));
        }

        edges
    }
}
