pub mod core;

#[cfg_attr(windows, path = "platform/windows.rs")]
#[cfg_attr(target_os = "linux", path = "platform/windows.rs")]
pub mod platform;
