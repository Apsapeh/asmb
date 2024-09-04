#[derive(serde::Deserialize)]
struct ConfigToml {
    pub name: String,
    pub kind: String,
    pub src: Option<String>,
    pub debug: Option<bool>,
    pub flags: Option<Vec<String>>,
    pub ignore: Option<Vec<String>>
}

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub kind: String,
    pub src: String,
    pub debug: bool,
    pub flags: Vec<String>,
    pub ignore: Vec<String>,
}

impl Config {
    pub fn from_file(file_name: &str) -> Self {
        let config_toml = std::fs::read_to_string(file_name).expect("Failed to read file");
        let config_toml: ConfigToml = toml::from_str(&config_toml).expect("Failed to parse TOML");

        let flags = config_toml.flags.unwrap_or_default();
        let ignore = config_toml.ignore.unwrap_or_default();

        let src = config_toml.src.unwrap_or("src".to_string());
        let debug = config_toml.debug.unwrap_or(false);


        Self {
            name: config_toml.name,
            kind: config_toml.kind,
            src,
            debug,
            flags,
            ignore,
        }
    }
}