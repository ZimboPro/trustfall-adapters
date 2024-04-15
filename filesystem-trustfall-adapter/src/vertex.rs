use std::path::PathBuf;

#[non_exhaustive]
#[derive(Debug, Clone, trustfall::provider::TrustfallEnumVertex)]
pub enum Vertex {
    File(PathBuf),
    Folder(PathBuf),
    Path(PathBuf),
}
