pub mod core;

#[cfg_attr(windows, path = "platform/windows.rs")]
#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
pub mod platform;

pub mod game;

pub mod entry;