mod format_config;
mod to_external_options;
mod to_oxfmt_options;

pub use format_config::{
    EndOfLineConfig, FormatConfig, OxfmtOverrideConfig, Oxfmtrc, SortPackageJsonConfig,
    SortPackageJsonUserConfig,
};
#[cfg(feature = "napi")]
pub use to_external_options::inject_oxfmt_plugin_payload;
pub use to_external_options::{
    inject_filepath, inject_parser, inject_tailwind_plugin_payload, to_prettier_options,
};
pub use to_oxfmt_options::{to_format_options, to_toml_options};
