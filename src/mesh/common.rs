use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Default)]
pub struct Edge {
    p: usize,
    q: usize,
    patches: Vec<usize>,
}

#[pymethods]
impl Edge {
    /// Construct an Edge from its vertices and patches
    #[new]
    pub fn new(p: usize, q: usize, patches: Vec<usize>) -> Edge {
        Edge { p, q, patches }
    }

    /// Get the patches.
    pub fn patches(&self) -> Vec<usize> {
        self.patches.clone()
    }

    /// (Python) Get the vertex by index.
    pub fn __getitem__(&self, index: usize) -> usize {
        self[index]
    }

    /// (Python) Set the vertex by index.
    pub fn __setitem__(&mut self, index: usize, value: usize) {
        self[index] = value;
    }
}

impl std::ops::Index<usize> for Edge {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.p,
            1 => &self.q,
            _ => panic!("index out of range"),
        }
    }
}

impl std::ops::IndexMut<usize> for Edge {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.p,
            1 => &mut self.q,
            _ => panic!("index out of range"),
        }
    }
}
