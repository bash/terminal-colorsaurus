#[cfg(not(target_os = "macos"))]
mod poll;
#[cfg(not(target_os = "macos"))]
pub(crate) use poll::*;
