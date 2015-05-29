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
//!    traceln!("2*2 = {}", double_var(2));
//!}
//! ```


///WARNING macro which uses ```TRACE!```
///
///Prints with the following format: ```file!:line! - WARNING: [Message]```
#[macro_export]
macro_rules! WARNING {
    ($($msg:expr),+) => {{ TRACE!(type => "WARNING", $($msg),+); }};
}

///ERROR macro which uses ```TRACE!```
///
///Prints with the following format: ```file!:line! - ERROR: [Message]```
#[macro_export]
macro_rules! ERROR {
    ($($msg:expr),+) => {{ TRACE!(type => "ERROR", $($msg),+); }};
}

///INFO macro which uses ```TRACE!```
///
///Prints with the following format: ```file!:line! - INFO: [Message]```
#[macro_export]
macro_rules! INFO {
    ($($msg:expr),+) => {{ TRACE!(type => "INFO", $($msg),+); }};
}

///DEBUG macro which uses ```TRACE!```
///
///Prints with the following format: ```file!:line! - DEBUG: [Message]```
#[macro_export]
macro_rules! DEBUG {
    ($($msg:expr),+) => {{ TRACE!(type => "DEBUG", $($msg),+); }};
}

///ENTER macro which uses ```TRACE!```
///
///Prints with the following format: ```file!:line! - ENTER: [arg_name=arg_value]```
///
///It is assumed to be used to wrap function call so all arguments are stringified, if any
#[macro_export]
macro_rules! ENTER {
    () => {{ TRACE!(type => "ENTER"); }};
    ($($msg:expr),+) => {{ TRACE!(type => "ENTER", $(format!("{}={}", stringify!($msg), $msg)),+); }};
}

///Trace macro which concats passed arguments into one string.
///
///Prints with the following format: ```file!:line! - [type:] [Message]```
///
///Usage:
///
///* ```TRACE!(type=>[TYPE], sep=>[String], [arg1, arg2, ..., argN])```
///* ```TRACE!(type=>[TYPE], [arg1, arg2, ..., argN])```
///* ```TRACE!(sep=>[String], [arg1, arg2, ..., argN])```
///* ```TRACE!([arg1, arg2, ..., argN])```
///* ```TRACE!()```
///
///Arguments must have ```fmt::Display``` trait.
#[macro_export]
macro_rules! TRACE {
    (type=>$tp:expr, sep=>$sep:expr, $msg:expr) => {{ println!("{}:{} - {}: {}", file!(), line!(), $tp, $msg); }};
    (type=>$tp:expr, sep=>$sep:expr, $($arg:expr),+) => {{ println!("{}:{} - {}: {}", file!(), line!(), $tp, [$(format!("{}", $arg),)+].connect($sep)); }};
    (type=>$tp:expr, $msg:expr) => {{ println!("{}:{} - {}: {}", file!(), line!(), $tp, $msg); }};
    (type=>$tp:expr, $($arg:expr),+) => {{ println!("{}:{} - {}: {}", file!(), line!(), $tp, [$(format!("{}", $arg),)+].connect(" ")); }};
    (type=>$tp:expr) => {{ println!("{}:{} - {}", file!(), line!(), $tp); }};
    (sep=>$sep:expr, $($arg:expr),+) => {{ println!("{}:{} - {}", file!(), line!(), [$(format!("{}", $arg),)+].connect($sep)); }};
    ($msg:expr) => {{ println!("{}:{} - {}", file!(), line!(), $msg); }};
    ($($arg:expr),+) => {{ println!("{}:{} - {}", file!(), line!(), [$(format!("{}", $arg),)+].connect(" ")); }};
    () => {{ println!("{}:{}", file!(), line!()); }};
}

///Debug trace macro like ```TRACE!```
///
///Prints with the following format: ```file!:line! - [type:] [Message]```
///
///Usage:
///
///* ```TRACE!(type=>[TYPE], sep=>[String], [arg1, arg2, ..., argN])```
///* ```TRACE!(type=>[TYPE], [arg1, arg2, ..., argN])```
///* ```TRACE!(sep=>[String], [arg1, arg2, ..., argN])```
///* ```TRACE!([arg1, arg2, ..., argN])```
///* ```TRACE!()```
///
///It is the same as ```TRACE!``` except that it prints with ```fmt::Debug``` trait.
#[macro_export]
macro_rules! DEBUG_TRACE {
    (type=>$tp:expr, sep=>$sep:expr, $msg:expr) => {{ println!("{}:{} - {}: {:?}", file!(), line!(), $tp, $msg); }};
    (type=>$tp:expr, sep=>$sep:expr, $($arg:expr),+) => {{ println!("{}:{} - {}: {}", file!(), line!(), $tp, [$(format!("{:?}", $arg),)+].connect($sep)); }};
    (type=>$tp:expr, $msg:expr) => {{ println!("{}:{} - {}: {:?}", file!(), line!(), $tp, $msg); }};
    (type=>$tp:expr, $($arg:expr),+) => {{ println!("{}:{} - {}: {}", file!(), line!(), $tp, [$(format!("{:?}", $arg),)+].connect(" ")); }};
    (type=>$tp:expr) => {{ println!("{}:{} - {}", file!(), line!(), $tp); }};
    (sep=>$sep:expr, $($arg:expr),+) => {{ println!("{}:{} - {}", file!(), line!(), [$(format!("{:?}", $arg),)+].connect($sep)); }};
    ($msg:expr) => {{ println!("{}:{} - {:?}", file!(), line!(), $msg); }};
    ($($arg:expr),+) => {{ println!("{}:{} - {}", file!(), line!(), [$(format!("{:?}", $arg),)+].connect(" ")); }};
    () => {{ println!("{}:{}", file!(), line!()); }};
}

///Trace macro like ```println```
///
///It uses ```format_args!``` for creating formatted string from passed arguments.
///
///Prints with the following format: ```file!:line! - [type:] [Message]```
///
#[macro_export]
macro_rules! traceln {
    (type=>$tp:expr, $($arg:tt)+) => {{ traceln!("{}: {}", $tp, format_args!($($arg)+)); }};
    (type=>$tp:expr) => {{ traceln!("{}", $tp); }};
    ($($arg:tt)+) => {{ println!("{}", format_args!("{}:{} - {}", file!(), line!(), format_args!($($arg)+))); }};
}

///Thread trace macro like ```println```
///
///It uses ```format_args!``` for creating formatted string from passed arguments.
///
///Prints with the following format: ```[thread_name] file!:line! - [type:] [Message]```
///
#[macro_export]
macro_rules! th_trace {
    (type=>$tp:expr, $($arg:tt)+) => {{ th_trace!("{}: {}", $tp, format_args!($($arg)+)); }};
    (type=>$tp:expr) => {{ th_trace!("{}", $tp); }};
    ($($arg:tt)+) => {{ println!("{}", format_args!("[{}] {}:{} - {}", std::thread::current().name().unwrap_or("none"), file!(), line!(), format_args!($($arg)+))); }};
}

///WARNING macro which uses ```traceln!```
///
///Prints with the following format: ```file!:line! - WARNING: [Message]```
#[macro_export]
macro_rules! warning {
    ($($msg:tt)+) => {{ traceln!(type=>"WARNING", $($msg)+); }};
}

///ERROR macro which uses ```traceln!```
///
///Prints with the following format: ```file!:line! - ERROR: [Message]```
#[macro_export]
macro_rules! error {
    ($($msg:tt)+) => {{ traceln!(type=>"ERROR", $($msg)+); }};
}

///INFO macro which uses ```traceln!```
///
///Prints with the following format: ```file!:line! - INFO: [Message]```
#[macro_export]
macro_rules! info {
    ($($msg:tt)+) => {{ traceln!(type=>"INFO", $($msg)+); }};
}

///DEBUG macro which uses ```traceln!```
///
///Prints with the following format: ```file!:line! - DEBUG: [Message]```
#[macro_export]
macro_rules! debug {
    ($($msg:tt)+) => {{ traceln!(type=>"DEBUG", $($msg)+); }};
}

///ENTER macro which uses ```traceln!```
///
///Prints with the following format: ```file!:line! - ENTER: [arg_name=arg_value]```
///
///It is assumed to be used to wrap function call so all arguments are stringified, if any
#[macro_export]
macro_rules! enter {
    () => {{ traceln!(type=>"ENTER"); }};
    ($($msg:expr),+) => {{ traceln!(type=>"ENTER", "{}", [$(format!("{}={}", stringify!($msg), $msg)),+].connect(" ")); }};
}

///Simplified trace macro
///
///Prints with the following format: ```file!:line! - [TYPE:] [Message]```
///
///Argument must have ```fmt::Display``` trait.
#[macro_export]
macro_rules! strace {
    (type=>$tp:expr, $msg:expr) => {{ println!("{}:{} - {}: {}", file!(), line!(), $tp, $msg); }};
    ($msg:expr) => {{ println!("{}:{} - {}", file!(), line!(), $msg); }};
}

///Macro to concat several arguments into one string.
///
///Arguments:
///
///* ```sep``` is a string which is used to separate arguments. Default is white-space.
///* ```formatter``` is a valid string to pass in ```format!``` . Default is ```"{}"```.
///
///Usage:
///
///* ```connect_args!(formatter=>[String], sep=>[String], [arg1, arg2, ..., argN])```
///* ```connect_args!(sep=>[String], [arg1, arg2, ..., argN])```
///* ```connect_args!(formatter=>[String], [arg1, arg2, ..., argN])```
///* ```connect_args!([arg1, arg2, ..., argN])```
#[macro_export]
macro_rules! connect_args {
    (formatter=>$fr:expr, sep=>$sep:expr, $($arg:expr),+) => { [$(format!($fr, $arg),)+].connect($sep) };
    (sep=>$sep:expr, $($arg:expr),+) => { [$(format!("{}", $arg),)+].connect($sep) };
    (formatter=>$fr:expr, $($arg:expr),+) => { [$(format!($fr, $arg),)+].connect(" ") };
    ($msg:expr) => { format!("{}", $msg) };
    ($($arg:expr),+) => { [$(format!("{}", $arg),)+].connect(" ") };
}
