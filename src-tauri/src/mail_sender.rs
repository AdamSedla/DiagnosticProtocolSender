use std::fs;
use std::path::PathBuf;

use lettre::message::Mailbox;
use lettre::message::{header::ContentType, Attachment, Body, MultiPart};
use lettre::{Address, Message, SmtpTransport, Transport};

use tauri_plugin_dialog::FilePath;

use crate::config::Config;
use crate::mail_list_utils;
use crate::mail_list_utils::Person;

use thiserror::Error;

use anyhow::Result;

#[derive(Error, Debug)]
pub enum MailSenderError {
    #[error("invalid file path")]
    InvalidFilePath,

    #[error("no recipients")]
    NoRecipients,

    #[error("no file")]
    NoFile,

    #[error("couldn't send email: {0}")]
    CouldntSendEmail(#[from] lettre::error::Error),

    #[error("InvalidMessage")]
    InvalidMessage,

    #[error("Couldn't open a remote connection to gmail")]
    NoRemoteConnection,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Recipient {
    pub name: String,
    pub mail: Address,
}

#[derive(Default, Debug)]
pub struct MailSender {
    people: Vec<Recipient>,
    files: Option<Vec<PathBuf>>,
}

impl MailSender {
    pub fn add_person(&mut self, person: Person) -> &mut Self {
        let person_parsed = Recipient {
            name: person.name,
            mail: person.mail.parse().unwrap(),
        };

        self.people.push(person_parsed);

        self
    }

    pub fn remove_person(&mut self, person: Person) -> &mut Self {
        let person_parsed = Recipient {
            name: person.name,
            mail: person.mail.parse().unwrap(),
        };

        self.people.retain(|x| *x != person_parsed);

        self
    }

    pub fn add_file(&mut self, vec_path: Vec<FilePath>) -> Result<(), MailSenderError> {
        self.files = Some(vec![]);

        for file in vec_path {
            let path = file
                .into_path()
                .map_err(|_| MailSenderError::InvalidFilePath)?;

            if !path.is_file() {
                return Err(MailSenderError::InvalidFilePath);
            }

            self.files.as_mut().unwrap().push(path);
        }

        Ok(())
    }

    pub fn send(
        &mut self,
        other_mail_list: Vec<mail_list_utils::Person>,
        config: Config,
    ) -> Result<()> {
        other_mail_list.iter().for_each(|person| {
            self.add_person(person.clone());
        });

        if self.people.is_empty() {
            return Err(MailSenderError::NoRecipients.into());
        }
        if self.files.is_none() {
            return Err(MailSenderError::NoFile.into());
        }

        let mut message_builder = Message::builder();

        //sender
        message_builder = message_builder.from(Mailbox::new(
            Some(config.sender_name().to_string()),
            config.sender_mail().parse()?,
        ));

        //recipient
        message_builder = self
            .people
            .iter()
            .fold(message_builder, |message_builder, recipient| {
                message_builder.to(Mailbox::new(
                    Some(recipient.name.clone()),
                    recipient.mail.clone(),
                ))
            });

        //subject
        message_builder = message_builder.subject(config.get_title());

        //attachments
        let mut attachment_multipart = MultiPart::mixed().build();

        for file_path in self.files.as_ref().unwrap() {
            let file = fs::read(file_path).map_err(|_| MailSenderError::InvalidFilePath)?;

            let mime_type = mime_guess::from_path(file_path);

            let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();

            attachment_multipart =
                attachment_multipart
                    .clone()
                    .singlepart(Attachment::new(file_name).body(
                        Body::new(file),
                        ContentType::parse(mime_type.first().unwrap().essence_str())?,
                    ));
        }

        let message = message_builder.multipart(attachment_multipart);

        //get credentials
        let creds = config.credentials();

        // open a remote connection to gmail
        let mailer = SmtpTransport::relay(config.smtp_transport())
            .map_err(|_| MailSenderError::NoRemoteConnection)?
            .credentials(creds)
            .build();

        //send the email
        mailer.send(&message.map_err(|_| MailSenderError::InvalidMessage)?)?;

        Ok(())
    }

    pub fn file_is_valid(&self) -> bool {
        self.files.is_some()
    }

    pub fn person_list_is_valid(&self) -> bool {
        !self.people.is_empty()
    }

    pub fn send_feedback(text: String, config: Config) -> Result<()> {
        let mut message_builder = Message::builder();

        //sender
        message_builder = message_builder.from(Mailbox::new(
            Some(config.sender_name().to_string()),
            config.sender_mail().parse()?,
        ));

        //recepient
        message_builder = message_builder.to(Mailbox::new(
            Some(config.feedback_recepient().to_string()),
            config.feedback_mail().parse()?,
        ));

        //subject
        message_builder = message_builder.subject(config.feedback_subject());

        //body
        let message = message_builder.body(text);

        //get credentials
        let creds = config.credentials();

        // open a remote connection to gmail
        let mailer = SmtpTransport::relay(config.smtp_transport())
            .map_err(|_| MailSenderError::NoRemoteConnection)?
            .credentials(creds)
            .build();

        //send the email
        mailer.send(&message.map_err(|_| MailSenderError::InvalidMessage)?)?;

        Ok(())
    }

    pub fn clear(&mut self) {
        self.files = None;
        self.people.clear();
    }
}
