#[macro_export]
macro_rules! fmt_bytes {
    ($bytes:expr) => {{
        let mut bytes = $bytes as f64;
        let mut unit = 0;
        while bytes >= 1024.0 {
            unit += 1;
            unit %= 5;
            bytes /= 1024.0;
        }

        format!("{:.2} {}", bytes, ["B", "KB", "MB", "GB", "TB"][unit])
    }};
}