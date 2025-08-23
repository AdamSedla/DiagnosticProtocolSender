use crate::config::config;

pub struct RonUtils {}

impl RonUtils {
    pub fn load_config() -> config {
        let ron_string = std::fs::read_to_string("config.ron").unwrap();
        let result: config = ron::de::from_str(&ron_string).unwrap();
        result
    }
}
