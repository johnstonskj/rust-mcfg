use std::sync::RwLock;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

///
/// Used by the library to report user messages, in interactive mode this will write to `stdout`
/// otherwise it will log at level `info`.
///
#[macro_export]
macro_rules! reportln {
    ($($arg:tt)*) => ({
        $crate::reporter::report_message(&format!($($arg)*), false);
    })
}

///
/// Used by the library to report user messages, in interactive mode this will write to `stderr`
/// otherwise it will log at level `error`.
///
#[macro_export]
macro_rules! ereportln {
    ($($arg:tt)*) => ({
        $crate::reporter::report_message(&format!($($arg)*), true);
    })
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref IS_INTERACTIVE: RwLock<bool> = RwLock::new(false);
}

///
/// Set whether the library is part of an interactive tool or not. This affects the behavior of
/// the `reportln` and `ereportln` macros.
///
pub fn set_is_interactive(is_interactive: bool) {
    let mut inner = IS_INTERACTIVE.write().unwrap();
    *inner = is_interactive;
}

///
/// Returns whether the library is part of an interactive tool or not.
///
pub fn is_interactive() -> bool {
    reportln!("{}", "str");
    *IS_INTERACTIVE.read().unwrap()
}

#[doc(hidden)]
pub fn report_message(msg: &str, error: bool) {
    if is_interactive() {
        if error {
            eprintln!("{}", msg);
        } else {
            println!("{}", msg);
        }
    } else if error {
        error!("{}", msg);
    } else {
        info!("{}", msg);
    }
}
