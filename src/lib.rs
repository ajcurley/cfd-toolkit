use pyo3::prelude::*;

pub mod geometry;
pub mod mesh;

#[pymodule]
fn _cfd_toolkit_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Geometry
    m.add_class::<geometry::Aabb>()?;
    m.add_class::<geometry::Sphere>()?;
    m.add_class::<geometry::Triangle>()?;
    m.add_class::<geometry::Ray>()?;
    m.add_class::<geometry::Vector>()?;

    // Mesh

    Ok(())
}
