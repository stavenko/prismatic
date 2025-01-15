use super::path_trait::Path;

pub struct SuperPath<T: Path>(pub(super) T);
