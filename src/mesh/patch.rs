/// Patch for a group of faces
pub struct Patch {
    name: String,
}

impl Patch {
    /// Construct a Patch from its name
    pub fn new(name: &str) -> Patch {
        Patch { name: name.clone() }
    }
}
