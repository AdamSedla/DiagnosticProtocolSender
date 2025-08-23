use lettre::transport::smtp::authentication::Credentials;
use serde::{Deserialize, Serialize};

use crate::ron_utils::RonUtils;

#[derive(Serialize, Deserialize, Debug)]
pub struct config {
    sender_name: String,
    sender_mail: String,
    sender_password: String,
    title: String,
    attachment_name: String,
    smtp_transport: String,
}

impl config {
    pub fn load_config() -> config {
        RonUtils::load_config()
    }
    pub fn get_sender_name(&self) -> String {
        self.sender_name.clone()
    }
    pub fn get_sender_mail(&self) -> String {
        self.sender_mail.clone()
    }
    pub fn get_credentials(&self) -> Credentials {
        Credentials::new(self.sender_mail.clone(), self.sender_password.clone())
    }
    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn get_attachment_name(&self) -> String {
        self.attachment_name.clone()
    }
    pub fn get_smtp_transport(&self) -> &str {
        &self.smtp_transport
    }
}
