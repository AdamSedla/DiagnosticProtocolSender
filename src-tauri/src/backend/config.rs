use lettre::transport::smtp::authentication::Credentials;
use serde::{Deserialize, Serialize};

//---------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    sender_name: String,
    sender_mail: String,
    sender_password: String,
    title: String,
    smtp_transport: String,
    feedback_mail: String,
    feedback_recepient: String,
    feedback_subject: String,
    settings_password: String,
}

impl Config {
    pub fn save_config(&self) {
        let ron_string =
            ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap();

        std::fs::write("config.ron", ron_string).unwrap();
    }
    pub fn load_config() -> Config {
        let ron_string = std::fs::read_to_string("config.ron").unwrap();
        let result: Config = ron::de::from_str(&ron_string).unwrap();
        result
    }
    pub fn sender_name(&self) -> &str {
        &self.sender_name
    }
    pub fn save_sender_name(&mut self, text: String) {
        self.sender_name = text;
    }
    pub fn sender_mail(&self) -> &str {
        &self.sender_mail
    }
    pub fn save_sender_mail(&mut self, text: String) {
        self.sender_mail = text;
    }
    pub fn sender_password(&self) -> &str {
        &self.sender_password
    }
    pub fn save_sender_password(&mut self, text: String) {
        self.sender_password = text;
    }
    pub fn credentials(&self) -> Credentials {
        Credentials::new(self.sender_mail.clone(), self.sender_password.clone())
    }
    pub fn title(&self) -> &str {
        &self.title
    }
    pub fn save_title(&mut self, text: String) {
        self.title = text;
    }
    pub fn smtp_transport(&self) -> &str {
        &self.smtp_transport
    }
    pub fn save_smtp_transport(&mut self, text: String) {
        self.smtp_transport = text;
    }
    pub fn feedback_mail(&self) -> &str {
        &self.feedback_mail
    }
    pub fn save_feedback_mail(&mut self, text: String) {
        self.feedback_mail = text;
    }
    pub fn feedback_recepient(&self) -> &str {
        &self.feedback_recepient
    }
    pub fn save_feedback_recepient(&mut self, text: String) {
        self.feedback_recepient = text;
    }
    pub fn feedback_subject(&self) -> &str {
        &self.feedback_subject
    }
    pub fn save_feedback_subject(&mut self, text: String) {
        self.feedback_subject = text;
    }
    pub fn settings_password(&self) -> &str {
        &self.settings_password
    }
    pub fn save_settings_password(&mut self, text: String) {
        self.settings_password = text;
    }
    pub fn settings_password_check(&self, password: &str) -> bool {
        self.settings_password == password
    }
}
