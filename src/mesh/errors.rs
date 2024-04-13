#[derive(Debug, Clone)]
pub struct NonManifoldError;

impl std::fmt::Display for NonManifoldError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "non-manifold edge found")
    }
}
