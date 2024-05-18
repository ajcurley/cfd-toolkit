use meshx::mesh::half_edge::HeMesh;
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
}
