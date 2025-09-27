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
    pub fn attachment_name(&self) -> &str {
        &self.attachment_name
    }
    pub fn smtp_transport(&self) -> &str {
        &self.smtp_transport
    }
}
