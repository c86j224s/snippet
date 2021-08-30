#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub connection_string: String
}

impl Config {
    pub fn from_file(config_file_name: &str) -> Result<Config, std::io::Error> {
        let data = std::fs::read_to_string(config_file_name)?;

        let instance = serde_json::from_str(&data)?;

        Ok(instance)
    }
}