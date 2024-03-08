#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key, $value);
            )*
            map
        }
    };
}

#[macro_export]
macro_rules! map_str {
    ($($key:expr => $value:expr),*) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $(
                map.insert($key.to_string(), $value.to_string());
            )*
            map
        }
    };
}