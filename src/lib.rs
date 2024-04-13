use pyo3::prelude::*;

pub mod geometry;

#[pymodule]
fn _cfd_toolkit_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Geometry
    m.add_class::<geometry::Vector>()?;

    Ok(())
}
