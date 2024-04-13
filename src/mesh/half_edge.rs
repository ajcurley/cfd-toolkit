use pyo3::prelude::*;

use crate::geometry::Vector;

#[pyclass]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct VertexHandle(usize);

#[pymethods]
impl VertexHandle {
    /// Construct a VertexHandle from its id
    #[new]
    pub fn new(id: usize) -> VertexHandle {
        VertexHandle(id)
    }

    /// Get the handle id
    pub fn id(&self) -> usize {
        self.0
    }
}

#[pyclass]
#[derive(Debug, Copy, Clone, Default)]
pub struct Vertex {
    point: Vector,
    half_edge: HalfEdgeHandle,
}

#[pyclass]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct FaceHandle(usize);

#[pymethods]
impl FaceHandle {
    /// Construct a FaceHandle from its id
    #[new]
    pub fn new(id: usize) -> FaceHandle {
        FaceHandle(id)
    }

    /// Get the handle id
    pub fn id(&self) -> usize {
        self.0
    }
}

#[pyclass]
#[derive(Debug, Copy, Clone, Default)]
pub struct Face {
    half_edge: HalfEdgeHandle,
    patch: Option<PatchHandle>,
}

#[pyclass]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct HalfEdgeHandle(usize);

#[pymethods]
impl HalfEdgeHandle {
    /// Construct a HalfEdgeHandle from its id
    #[new]
    pub fn new(id: usize) -> HalfEdgeHandle {
        HalfEdgeHandle(id)
    }

    /// Get the handle id
    pub fn id(&self) -> usize {
        self.0
    }
}

#[pyclass]
#[derive(Debug, Copy, Clone, Default)]
pub struct HalfEdge {
    origin: VertexHandle,
    face: FaceHandle,
    prev: HalfEdgeHandle,
    next: HalfEdgeHandle,
    twin: Option<HalfEdgeHandle>,
}

#[pyclass]
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct PatchHandle(usize);

#[pymethods]
impl PatchHandle {
    /// Construct a PatchHandle from its id
    #[new]
    pub fn new(id: usize) -> PatchHandle {
        PatchHandle(id)
    }

    /// Get the handle id
    pub fn id(&self) -> usize {
        self.0
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Patch {
    name: String,
}
