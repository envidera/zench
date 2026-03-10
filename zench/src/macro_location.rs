/// location! Returns a string with the file and line number of the **call site**.
///
/// **Note:** The function that calls this macro should be annotated with `#[track_caller]`
/// in order to get the correct caller location.
///
/// This macro uses `std::panic::Location::caller` to capture the location
/// where it was invoked, making it useful for debugging, logging, or
/// error messages.
///
/// # Example
///
/// ```rust,ignore
///
/// #[track_caller]
/// fn log_location() {
///     let location = location!();
///     println!("This code is running at {}", location);
/// }
///
/// log_location();
/// // Output might be: "src/main.rs:42"
/// ```
///
/// # Notes
///
/// - Expands to a `String` containing the file path and line number.
/// - Useful for quickly identifying the caller location without manually passing `file!()` and `line!()`.
///
/// # Panics
///
/// This macro does not panic.
#[macro_export]
#[doc(hidden)]
macro_rules! location {
    () => {{
        let caller = std::panic::Location::caller();
        format!("{}:{}:{}", caller.file(), caller.line(), caller.column())
    }};
}
