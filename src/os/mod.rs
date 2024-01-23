#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub(crate) use macos::*;
#[cfg(unix)]
mod unix;
#[cfg(all(unix, not(target_os = "macos")))]
pub(crate) use unix::*;
