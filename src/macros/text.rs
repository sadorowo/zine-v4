#[macro_export]
macro_rules! truncate {
    ($text:expr, $max_length:expr) => {{
        let text = $text;
        let max_length = $max_length;
        if text.len() > max_length {
            format!("{}...", &text[..max_length - 3])
        } else {
            text.to_string()
        }
    }};
}

#[macro_export]
macro_rules! no_md {
    ($text:expr) => {{
        let text = $text;
        text.replace("*", "\\*")
            .replace("_", "\\_")
            .replace("~", "\\~")
            .replace("`", "\\`")
    }};
}

#[macro_export]
macro_rules! code_block {
    ($text:expr) => {{
        let text = $text;
        format!("```{}```", text)
    }};

    ($lang:expr, $text:expr) => {{
        let text = $text;
        let lang = $lang;
        format!("```{}\n{}```", lang, text)
    }};
}

#[macro_export]
macro_rules! to_timestamp {
    ($timestamp:expr) => {{
        let timestamp = $timestamp;
        format!("<t:{}:R>", timestamp.timestamp())
    }};
    
    ($timestamp:expr, $format:expr) => {{
        let timestamp = $timestamp;
        let format = $format;
        format!("<t:{}:{}>", timestamp.timestamp(), format)
    }};
}