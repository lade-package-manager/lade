#[macro_export]
/// A macro to print error messages with a specific format.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// err!("Error occurred", "Something went wrong");
/// ```
///
/// This will print:
///
/// ```text
/// >>> Error occurred
/// Something went wrong
/// ```
///
/// You can also use it with a single argument:
///
/// ```
/// err!("Error occurred");
/// ```
///
/// This will print:
///
/// ```text
/// >>> Error occurred
/// ```
///
/// This will produce a log entry with the current date and time, the error message, and the error code.
macro_rules! err {
    ($err: expr, $msg: expr) => {{
        println!("\x1b[31;1m>>>\x1b[0m \x1b[1m{}\x1b[0m", $err);
        println!("{}", $msg);
    }};

    ($err: expr) => {
        println!("\x1b[31;1m>>>\x1b[0m \x1b[1m{}\x1b[0m", $err);
    };
}

#[macro_export]
macro_rules! err_with_fmt {
    ($($arg:tt)*) => {
        let fmt = format!($($arg)*);
        println!("\x1b[31;1m>>>\x1b[0m \x1b[1m{}\x1b[0m", fmt);
    };
}

#[macro_export]
/// A macro to print an informational message to the console with a specific format.
/// The message is prefixed with a green arrow (>>>), and the message itself is bold.
///
/// # Examples
///
/// ```
/// info!("This is an informational message.");
/// ```
///
/// This will print:
///
/// ```
/// >>> This is an informational message.
/// ```
///
/// This will produce a log entry with the current date and time, the error message, and the error code.
macro_rules! info {
    ($($arg:tt)*) => {
        let fmt = format!($($arg)*);
        println!("\x1b[32;1m>>> \x1b[0m\x1b[1m{fmt}\x1b[0m");
    };
}

#[macro_export]
/// Macro to terminate the program with a specified exit code.
///
/// # Examples
///
/// Terminate the program with exit code 1:
/// ```rust,should_panic
/// crash!();
/// ```
///
/// Terminate the program with a custom exit code:
/// ```rust,should_panic
/// crash!(42);
/// ```
///
/// # Parameters
///
/// - `()` - Terminates the program with exit code 1.
/// - `($exit: expr)` - Terminates the program with the specified exit code.
macro_rules! crash {
    () => {
        std::process::exit(1);
    };
    ($exit: expr) => {
        std::process::exit($exit);
    };
}

#[macro_export]
/// Macro to write a log entry to a log file.
///
/// This macro takes a single expression as input, formats it, and appends it to a log file.
/// The log file path is determined by the `crate::paths::lade_log_path` function.
///
/// # Arguments
///
/// * `$content` - The content to be logged. This expression will be formatted and written to the log file.
///
/// # Example
///
/// ```rust
/// write_log!("This is a log entry");
/// ```
///
/// # Errors
///
/// If the log file cannot be opened or written to, an error message will be logged and the program will crash.
macro_rules! write_log {
    ($content: expr) => {{
        use crate::{crash, err};
        use std::io::Write;
        let path = $crate::paths::lade_log_path();
        let content = format!("----------------\n{}\n", $content);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(path)
            .unwrap_or_else(|e| {
                err!("Failed to open log file", e);
                crash!();
            });

        writeln!(file, "{}", content).unwrap_or_else(|e| {
            err!("Failed to write log for log file", e);
            crash!();
        });
    }};
}

#[macro_export]

/// Logs an error message along with an error code and the current date and time.
///
/// # Arguments
///
/// * `$error` - The error message to log.
/// * `$error_code` - The error code associated with the error.
///
/// # Example
///
/// ```rust
/// log!("An error occurred", "Unknown error");
/// ```
///
/// This will produce a log entry with the current date and time, the error message, and the error code.
macro_rules! log {
    ($error: expr, $error_code: expr) => {{
        use crate::write_log;
        write_log!(format!(
            "date: {}\nerror: {}\nerror_code: {}",
            chrono::Local::now(),
            $error,
            $error_code
        ));
    }};
}

#[macro_export]
/// Macro to handle errors by logging them and crashing the program.
///
/// # Usage
///
/// There are three forms of this macro:
///
/// 1. `error!($error, $error_code)`
///
/// Logs the error message and error code, then crashes the program.
///
/// - `$error`: The error message to log.
/// - `$error_code`: The error code to log.
///
/// 2. `error!($error)`
///
/// Logs the error message twice (once as the error message and once as the error code), then crashes the program.
///
/// - `$error`: The error message to log.
///
/// 3. `error!($error, $error_code, $exit)`
///
/// Logs the error message and error code, then crashes the program with a specific exit code.
///
/// - `$error`: The error message to log.
/// - `$error_code`: The error code to log.
/// - `$exit`: The exit code to use when crashing the program.
///
/// # Examples
///
/// ```rust
/// error!("An error occurred", "Unknow error");
/// error!("An error occurred");
/// error!("An error occurred", "Unknown error", 1);
/// ```
macro_rules! error {
    ($error: expr, $error_code: expr) => {{
        use $crate::{crash, err, write_log};

        err!($error);
        write_log!(format!(
            "date: {}\nerror: {}\nerror_code: {}",
            chrono::Local::now(),
            $error,
            $error_code
        ));
        crash!();
    }};

    ($error: expr) => {{
        use $crate::{crash, err, write_log};

        err!($error);
        write_log!(format!(
            "date: {}\nerror: {}\nerror_code: {}",
            chrono::Local::now(),
            $error,
            $error
        ));
        crash!();
    }};

    ($error: expr, $error_code: expr, $exit: expr) => {{
        use crate::{crash, err, write_log};

        err!($error);
        write_log!(format!(
            "date: {}\nerror: {}\nerror_code: {}",
            chrono::Local::now(),
            $error,
            $error_code
        ));
        crash!($exit);
    }};
}

pub trait UnwrapOrCrash<T, E> {
    fn unwrap_or_crash<F: FnOnce(E)>(self, f: F) -> T;
    fn unwrap_or_crash_by_status<F: FnOnce(E)>(self, status: i32, f: F) -> T;
}

impl<T, E> UnwrapOrCrash<T, E> for Result<T, E> {
    fn unwrap_or_crash<F: FnOnce(E)>(self, f: F) -> T {
        self.unwrap_or_else(|e| {
            f(e);
            crash!();
        })
    }

    fn unwrap_or_crash_by_status<F: FnOnce(E)>(self, status: i32, f: F) -> T {
        self.unwrap_or_else(|e| {
            f(e);
            crash!(status);
        })
    }
}
