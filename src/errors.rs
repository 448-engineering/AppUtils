/// The Result type with [UtilsError] as the [Err] value
pub type UtilsResult<T> = Result<T, UtilsError>;

/// Errors encountered using this crate
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, thiserror::Error)]
pub enum UtilsError {
    /// User Home Directory Not Found
    #[cfg(feature = "app_dir")]
    #[error("User Home Directory Not Found")]
    UserHomeDirNotFound,
    /// Path to home directory is not valid UTF8
    #[cfg(all(feature = "app_dir", feature = "utf8"))]
    #[error("Path to home directory is not valid UTF8")]
    UserHomeNotValidUtf8,
}
