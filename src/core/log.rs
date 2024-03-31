use colored::*;
use std::convert::Infallible;

#[cfg_attr(not(feature = "strings-assertions"), path = noasserts)]
mod asserts;
pub use asserts::*;

#[repr(u8)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Fatal = 0,
    Error = 1,
    Warn = 2,
    Info = 3,
    Debug = 4,
    Trace = 5,
}
impl LogLevel {
    #[inline]
    pub fn color(&self) -> Color {
        match self {
            Self::Fatal => Color::BrightRed,
            Self::Error => Color::Red,
            Self::Warn => Color::Yellow,
            Self::Info => Color::Green,
            Self::Debug => Color::Blue,
            Self::Trace => Color::White,
        }
    }
}
impl std::fmt::Debug for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Fatal => "[ Fatal ]".on_bright_red(),
                Self::Error => "[ Error ]".bright_red(),
                Self::Warn => "[  Warn ]".bright_yellow(),
                Self::Info => "[  Info ]".bright_green(),
                Self::Debug => "[ Debug ]".bright_blue(),
                Self::Trace => "[ Trace ]".bright_white(),
            }
            .bold()
        )
    }
}

#[allow(dead_code)]
fn init() -> Result<(), Infallible> {
    // TODO: create log file.
    Ok(())
}
#[allow(dead_code)]
fn close() {
    // TODO: cleanup logging/write queued entries.
}

#[macro_export(local_inner_macros)]
macro_rules! log {
    ($lvl:expr,$($args:tt)*) => {
        use colored::*;
        if $lvl < $crate::core::log::LogLevel::Warn {
            std::eprintln!("{:?}: {}", $lvl, std::format!($($args)*).color($lvl.color()).underline());
        } else {
            std::println!("{:?}: {}", $lvl, std::format!($($args)*).color($lvl.color()).underline());
        }
    };
}

#[macro_export(local_inner_macros)]
macro_rules! fatal {
    ($($args:tt)*) => {
        log!($crate::core::log::LogLevel::Fatal, $($args)*); // Logs a fatal-level message.
    };
}

#[macro_export(local_inner_macros)]
macro_rules! error {
    ($($args:tt)*) => {
        log!($crate::core::log::LogLevel::Error, $($args)*); // Logs a error-level message.
    };
}

#[macro_export(local_inner_macros)]
macro_rules! warn {
    ($($args:tt)*) => {
        log!($crate::core::log::LogLevel::Warn, $($args)*); // Logs a warning-level message.
    };
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    ($($args:tt)*) => {
        log!($crate::core::log::LogLevel::Info, $($args)*); // Logs a info-level message.
    };
}

#[cfg(debug_assertions)]
#[macro_export(local_inner_macros)]
macro_rules! debug {
    ($($args:tt)*) => {
        log!($crate::core::log::LogLevel::Debug, $($args)*); // Logs a debug-level message.
    };
}
#[cfg(not(debug_assertions))]
#[macro_export(local_inner_macros)]
macro_rules! debug {
    ($($args:tt)*) => {};
}

#[cfg(all(debug_assertions, not(feature = "no-trace")))]
#[macro_export(local_inner_macros)]
macro_rules! trace {
    ($($args:tt)*) => {
        log!($crate::core::log::LogLevel::Trace, $($args)*); // Logs a trace-level message.
    };
}
#[cfg(any(not(debug_assertions), feature = "no-trace"))]
#[macro_export(local_inner_macros)]
macro_rules! trace {
    ($($args:tt)*) => {};
}
