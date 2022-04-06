#[macro_export]
macro_rules! custom_err {
    ($error_level:expr, $($arg:tt)+) => {
        {
            let path = format!("{}:{}", file!(), line!());
            let message = format!($($arg)+);
            $crate::emit_log(String::from($error_level), message, path)
        }
    }
}

#[macro_export]
macro_rules! severe {
    ($($arg:tt)+) => {
        $crate::custom_err!("Severe", $($arg)+)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {
        $crate::custom_err!("Error", $($arg)+)
    };
}

#[macro_export]
macro_rules! warning {
    ($($arg:tt)+) => {
        $crate::custom_err!("Warning", $($arg)+)
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {
        $crate::custom_err!("Info", $($arg)+)
    };
}

#[macro_export]
macro_rules! verbose {
    ($($arg:tt)+) => {
        $crate::custom_err!("Verbose", $($arg)+)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => {
        $crate::custom_err!("Debug", $($arg)+)
    };
}
