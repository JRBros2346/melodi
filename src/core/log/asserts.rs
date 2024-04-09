pub fn report_assertion_failure(expr: &str, msg: &str, file: &str, line: u32) {
    crate::log!(
        super::LogLevel::Fatal,
        "
    Assertion: {}
    Message: '{}'
    Location: {}:{}",
        expr,
        msg,
        file,
        line
    );
}

#[macro_export]
macro_rules! assert {
    ($expr:expr) => {
        if ($expr) {
        } else {
            $crate::core::log::report_assertion_failure(stringify!($expr), "", file!(), line!());
            std::assert!($expr);
        }
    };
}

#[macro_export]
macro_rules! assert_msg {
    ($expr:expr, $msg:literal) => {
        if ($expr) {
        } else {
            $crate::core::log::report_assertion_failure(stringify!($expr), $msg, file!(), line!());
            std::assert!($expr);
        }
    };
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! assert_debug {
    ($expr:expr) => {
        if ($expr) {
        } else {
            $crate::core::log::report_assertion_failure(stringify!($expr), "", file!(), line!());
            std::assert!($expr);
        }
    };
}
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! asserts_debug {
    ($expr:expr) => {}; // Does nothing at all
}
