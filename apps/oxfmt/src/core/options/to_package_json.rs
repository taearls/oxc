use super::super::oxfmtrc::{FormatConfig, SortPackageJsonConfig, SortPackageJsonUserConfig};

/// Convert `FormatConfig` into `sort_package_json::SortOptions`.
///
/// `package.json` sorting is opt-out: when `sort_package_json` is unset,
/// it defaults to enabled with default options. Returns `None` only when
/// the user explicitly sets `sort_package_json: false`.
pub fn to_package_json(config: &FormatConfig) -> Option<sort_package_json::SortOptions> {
    config.sort_package_json.as_ref().map_or_else(
        || Some(SortPackageJsonConfig::default().to_sort_options()),
        SortPackageJsonUserConfig::to_sort_options,
    )
}
