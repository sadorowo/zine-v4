use crate::utils::framework::Context;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct LanguageHandler<'a> {
    pub locale: &'a str
}

impl<'a> LanguageHandler<'a> {
    pub fn from_context(ctx: Context<'a>) -> Self {
        Self { locale: ctx.locale().unwrap_or("en-US") }
    }

    pub fn no_context() -> Self {
        Self { locale: "en-US" }
    }

    pub fn list_language_files() -> Vec<String> {
        let mut language_files: Vec<String> = Vec::new();

        let paths = std::fs::read_dir("src/language").unwrap();
        for path in paths {
            let file_name = path.unwrap().file_name();
            let file_name = file_name.to_str().unwrap();

            if !file_name.ends_with(".toml") {
                continue;
            }

            let path_name = file_name.split(".").collect::<Vec<&str>>();
            language_files.push(path_name[0].to_string());
        }

        language_files
    }

    pub fn get_file_from(language: String) -> String {
        format!("src/language/{}.toml", language)
    }

    pub fn parse_language_file_from(language: String) -> toml::Value {
        let language_file = std::fs::read_to_string(Self::get_file_from(language)).unwrap();
        let language_file: toml::Value = toml::from_str(&language_file).unwrap();

        language_file
    }

    pub fn translate_from(language: String, key: String) -> String {
        let language_file = Self::parse_language_file_from(language);

        let key = key.split(".").collect::<Vec<&str>>();
        let mut key = key.iter();
        let mut value = language_file.get(key.next().unwrap()).unwrap();

        for key in key {
            let option = value.get(key);
            if option.is_none() {
                return format!("{}: {}", key, "not found");
            }

            value = option.unwrap();
        }

        value.as_str().unwrap().to_string()
    }

    pub fn translate(&self, key: &str) -> String {
        Self::translate_from(self.locale.to_string(), key.to_string())
    }

    pub fn translate_v(&self, key: &str, variables: HashMap<String, String>) -> String {
        let mut value = self.translate(key);
        for (from, to) in variables {
            value = value.replace(&format!("{{{}}}", from), &to);
        }

        value
    }

    pub fn get_localizations(&'a self, key: &'a str) -> HashMap<String, String> {
        let mut map: HashMap<String, String> = HashMap::new();
        let language_files = Self::list_language_files();
        let raw_key = key.split(".").collect::<Vec<&str>>();
        let default_value = &toml::Value::String("unknown".to_string());

        for language_file in language_files {
            let parsed_language_file = Self::parse_language_file_from(language_file.clone());

            let mut description = parsed_language_file
                .get(raw_key[0])
                .unwrap();

            for part in raw_key[1..].to_vec() {
                description = description.get(part).unwrap_or(default_value);

                if description.is_str() {
                    map.insert(language_file.clone(), description
                        .as_str()
                        .unwrap()
                        .to_string()
                    );
                    continue;
                }
            }
        }

        map
    }
    
    pub fn translate_bool(&self, value: bool) -> String {
        match value {
            true => self.translate("bool.true"),
            false => self.translate("bool.false")
        }
    }
}