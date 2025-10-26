use lettre::transport::smtp::authentication::Credentials;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    sender_name: String,
    sender_mail: String,
    sender_password: String,
    title: String,
    smtp_transport: String,
}

impl Config {
    pub fn load_config() -> Config {
        let ron_string = std::fs::read_to_string("Config.ron").unwrap();
        let result: Config = ron::de::from_str(&ron_string).unwrap();
        result
    }
    pub fn sender_name(&self) -> &str {
        &self.sender_name
    }
    pub fn sender_mail(&self) -> &str {
        &self.sender_mail
    }
    pub fn credentials(&self) -> Credentials {
        Credentials::new(self.sender_mail.clone(), self.sender_password.clone())
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn smtp_transport(&self) -> &str {
        &self.smtp_transport
    }
}
