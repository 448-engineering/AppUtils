/// Count the number of items inside a macro
#[macro_export]
macro_rules! items_counter {
    ($($name:expr),* $(,)?) => {
        {
           [$($name,)*].len()
        }
    }
}

/// Include asset in UTF-8 format at compile time
#[macro_export]
macro_rules! from_manifest_dir {
    ($value:expr) => {{
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $value))
    }};
}
