/// Get path to user's home directory and append a new file path.
/// This returns the [std::path::Path] of the final path
#[cfg(feature = "app_dir")]
#[macro_export]
macro_rules! app_dir {
    ($value:expr) => {{
        use directories::UserDirs;
        use std::path::PathBuf;

        if let Some(user_dirs) = UserDirs::new() {
            let mut path = PathBuf::new();
            path.push(user_dirs.home_dir());
            path.push($value);

            Ok(path)
        } else {
            Err(UtilsError::UserHomeDirNotFound)
        }
    }};
}

/// Get path to user's home directory and append a new file path.
/// This returns the `camino::Utf8PathBuf` of the final path
#[cfg(all(feature = "app_dir", feature = "utf8"))]
#[macro_export]
macro_rules! app_dir_utf8 {
    ($value:expr) => {{
        use camino::Utf8PathBuf;
        use directories::UserDirs;

        if let Some(user_dirs) = UserDirs::new() {
            if let Some(valid_path) = camino::Utf8Path::from_path(user_dirs.home_dir()) {
                let mut path = Utf8PathBuf::new();
                path.push(valid_path);
                path.push($value);

                Ok(path)
            } else {
                Err(UtilsError::UserHomeNotValidUtf8)
            }
        } else {
            Err(UtilsError::UserHomeDirNotFound)
        }
    }};
}
