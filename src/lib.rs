use pyo3::prelude::*;

pub mod geometry;

#[pymodule]
fn toussaint(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Geometry
    m.add_class::<geometry::Vector>()?;

    Ok(())
}
