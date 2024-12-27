//! Implements <https://wiki.tau.garden/cli-theme/>.

use std::env;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

pub(crate) fn cli_theme() -> Option<CliTheme> {
    let raw = env::var_os("CLITHEME")?;
    let preference = parse_preference(&raw);
    Some(CliTheme { preference })
}

fn parse_preference(raw: &OsStr) -> CliThemePreference {
    if raw == "dark" || raw.as_bytes().starts_with(b"dark:") {
        CliThemePreference::Dark
    } else if raw == "light" || raw.as_bytes().starts_with(b"light:") {
        CliThemePreference::Light
    } else {
        CliThemePreference::Auto
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub(crate) struct CliTheme {
    pub(crate) preference: CliThemePreference,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[allow(clippy::exhaustive_enums)]
pub(crate) enum CliThemePreference {
    Dark,
    Light,
    #[default]
    Auto,
}
