#[macro_export]
macro_rules! WARNING {
    ($($msg:expr),*) => {{ TRACE!(type => "WARNING", $($msg),*); }};
}

#[macro_export]
macro_rules! ERROR {
    ($($msg:expr),*) => {{ TRACE!(type => "ERROR", $($msg),*); }};
}

#[macro_export]
macro_rules! INFO {
    ($($msg:expr),*) => {{ TRACE!(type => "INFO", $($msg),*); }};
}

#[macro_export]
macro_rules! DEBUG {
    ($($msg:expr),*) => {{ TRACE!(type => "DEBUG", $($msg),*); }};
}

#[macro_export]
macro_rules! TRACE {
    (type=>$tp:expr, $($msg:expr),*) => {{ println!("{}:{} - [{}] {}", file!(), line!(), $tp, concat!($($msg, " "), *)); }};
    ($($msg:expr),*) => {{ println!("{}:{} - {}", file!(), line!(), concat!($($msg, " "), *)); }};
}
