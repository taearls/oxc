use std::path::{Path, PathBuf};

/// A supported configuration file discovered on disk.
///
/// The variant identifies which config source matched, while the contained path
/// points to the concrete file. Consumers can use [`DiscoveredConfigFile::path`]
/// when they only need the filesystem location.
#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub enum DiscoveredConfigFile {
    Json(PathBuf),
    Jsonc(PathBuf),
    Js(PathBuf),
}

impl DiscoveredConfigFile {
    pub fn path(&self) -> &Path {
        match self {
            Self::Json(path) | Self::Jsonc(path) | Self::Js(path) => path,
        }
    }
}
