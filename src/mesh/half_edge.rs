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

#[pymethods]
impl Vertex {
    /// Get the point
    pub fn point(&self) -> Vector {
        self.point
    }

    /// Get the half edge handle
    pub fn half_edge(&self) -> HalfEdgeHandle {
        self.half_edge
    }
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

#[pymethods]
impl Face {
    /// Get the half edge handle
    pub fn half_edge(&self) -> HalfEdgeHandle {
        self.half_edge
    }

    /// Get the patch handle
    pub fn patch(&self) -> Option<PatchHandle> {
        self.patch
    }
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

#[pymethods]
impl HalfEdge {
    /// Get the origin handle
    pub fn origin(&self) -> VertexHandle {
        self.origin
    }

    /// Get the face handle
    pub fn face(&self) -> FaceHandle {
        self.face
    }

    /// Get the prev half edge handle
    pub fn prev(&self) -> HalfEdgeHandle {
        self.prev
    }

    /// Get the next half edge handle
    pub fn next(&self) -> HalfEdgeHandle {
        self.next
    }

    /// Get the twin half edge handle
    pub fn twin(&self) -> Option<HalfEdgeHandle> {
        self.twin
    }

    /// Check if the half edge is a boundary (no twin)
    pub fn is_boundary(&self) -> bool {
        self.twin.is_none()
    }
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

#[pymethods]
impl Patch {
    /// Get the name
    pub fn name(&self) -> &str {
        &self.name
    }
}
