use meshx::mesh::half_edge::HeMesh;
use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;

use crate::mesh::Edge;

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

    /// Compute the feature edges.
    pub fn feature_edges(&self, angle: f64) -> Vec<Edge> {
        let mut edges = vec![];

        for (i, j) in self.data.feature_edges(angle) {
            let mut patches = vec![];

            let half_edge = self.data.half_edge(i);
            let face = half_edge.face();
            let next = half_edge.next();
            let p = half_edge.origin();
            let q = self.data.half_edge(next).origin();

            if let Some(patch) = self.data.face(face).patch() {
                patches.push(patch);
            }

            let half_edge = self.data.half_edge(j);
            let face = half_edge.face();

            if let Some(patch) = self.data.face(face).patch() {
                if !patches.contains(&patch) {
                    patches.push(patch);
                }
            }

            let edge = Edge::new(p, q, patches);
            edges.push(edge);
        }

        edges
    }
}
