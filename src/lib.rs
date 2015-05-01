//! A crate with printing macros.
//!
//! These macros aims to provide a simple and lazy way to print formatted traces.
//!
//! Simple example of usage:
//!
//! ```
//! #[macro_use(TRACE, ENTER)]
//! extern crate trace_macros;
//!
//!fn double_var(var: i32) -> i32 {
//!    ENTER!(var);
//!    var * var
//!}
//!
//!fn main() {
//!    ENTER!();
//!    TRACE!("I'm", "doing", "some", "lazy", "tracing");
//!    TRACE!("Result:", double_var(2));
//!}
//! ```

///WARNING macro
///
///Prints with the following format: file!:line! - WARNING: [Message]
#[macro_export]
macro_rules! WARNING {
    ($($msg:expr),+) => {{ TRACE!(type => "WARNING", $($msg),+); }};
}

///ERROR macro
///
///Prints with the following format: file!:line! - ERROR: [Message]
#[macro_export]
macro_rules! ERROR {
    ($($msg:expr),+) => {{ TRACE!(type => "ERROR", $($msg),+); }};
}

///INFO macro
///
///Prints with the following format: file!:line! - INFO: [Message]
#[macro_export]
macro_rules! INFO {
    ($($msg:expr),+) => {{ TRACE!(type => "INFO", $($msg),+); }};
}

///DEBUG macro
///
///Prints with the following format: file!:line! - DEBUG: [Message]
#[macro_export]
macro_rules! DEBUG {
    ($($msg:expr),+) => {{ TRACE!(type => "DEBUG", $($msg),+); }};
}

///ENTER macro
///
///Prints with the following format: file!:line! - ENTER: [arg_name=arg_value]
///
///It is assumed to be used to wrap function call so all arguments are stringified, if any
#[macro_export]
macro_rules! ENTER {
    () => {{ TRACE!(type => "ENTER"); }};
    ($($msg:expr),+) => {{ TRACE!(type => "ENTER", $(format!("{}={}", stringify!($msg), $msg)),+); }};
}

///Main trace macro.
///
///Prints with the following format: file!:line! - [TYPE:] [Message]
///
///[Message] consist of macro's arguments concated into one stringified
///and each separated by white-space.
///
///Arguments of macro MUST have a formated capabilities
///i.e. macro does not work with arrays and etc.
#[macro_export]
macro_rules! TRACE {
    (type=>$tp:expr, $($arg:expr),+) => {{ println!("{}:{} - {}: {}", file!(), line!(), $tp, [$(format!("{}", $arg),)+].connect(" ")); }};
    (type=>$tp:expr) => {{ println!("{}:{} - {}", file!(), line!(), $tp); }};
    ($($arg:expr),+) => {{ println!("{}:{} - {}", file!(), line!(), [$(format!("{}", $arg),)+].connect(" ")); }};
    () => {{ println!("{}:{}", file!(), line!()); }};
}
