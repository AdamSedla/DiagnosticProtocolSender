use std::fs;
use std::path::PathBuf;

use lettre::message::Mailbox;
use lettre::message::{header::ContentType, Attachment, Body, MultiPart};
use lettre::{Address, Message, SmtpTransport, Transport};

use tauri_plugin_dialog::FilePath;

use crate::config::config;

use thiserror::Error;

use anyhow::Result;

#[derive(Error, Debug)]
pub enum MailSenderError {
    #[error("invalid email")]
    InvalidEmail,

    #[error("not in database")]
    NotInDatabase,

    #[error("invalid file path")]
    InvalidFilePath,

    #[error("no receivers")]
    NoReceivers,

    #[error("no file")]
    NoFile,

    #[error("couldn't send email")]
    CouldntSendEmail,
}

#[derive(Debug, PartialEq)]
pub struct Receiver {
    pub name: String,
    pub mail: Address,
}

#[derive(Default, Debug)]
pub struct MailSender {
    people: Vec<Receiver>,
    file_path: Option<PathBuf>,
}

impl MailSender {
    pub fn add_person(&mut self, person: Receiver) -> &mut Self {
        self.people.push(person);

        self
    }

    pub fn remove_person(&mut self, person: Receiver) -> &mut Self {
        self.people.retain(|x| *x != person);

        self
    }

    pub fn add_file(&mut self, path: FilePath) -> Result<(), MailSenderError> {
        println!("File_path is: {path:?}");

        let path = path
            .into_path()
            .map_err(|_| MailSenderError::InvalidFilePath)?;

        if !path.is_file() {
            return Err(MailSenderError::InvalidFilePath);
        }

        self.file_path = Some(path);

        Ok(())
    }

    pub fn send(&self) -> Result<()> {
        if self.people.is_empty() {
            return Err(MailSenderError::NoReceivers.into());
        }
        if self.file_path.is_none() {
            return Err(MailSenderError::NoFile.into());
        }

        let config = config::load_config();

        let mut message_builder = Message::builder();

        //sender
        message_builder = message_builder.from(Mailbox::new(
            Some(config.get_sender_name()),
            config.get_sender_mail().parse()?,
        ));

        //receiver
        message_builder = self
            .people
            .iter()
            .fold(message_builder, |message_builder, receiver| {
                message_builder.to(Mailbox::new(
                    Some(receiver.name.clone()),
                    receiver.mail.clone(),
                ))
            });

        //subject
        message_builder = message_builder.subject(config.get_title());

        //attachment
        let file = fs::read(self.file_path.as_ref().unwrap())
            .map_err(|_| MailSenderError::InvalidFilePath)?;

        let message = message_builder.multipart(
            MultiPart::mixed().singlepart(
                Attachment::new(config.get_attachment_name().to_string())
                    .body(Body::new(file), ContentType::parse("application/pdf")?),
            ),
        );

        //get credentials
        let creds = config.get_credentials();

        // open a remote connection to gmail
        let mailer = SmtpTransport::relay(config.get_smtp_transport())
            .unwrap()
            .credentials(creds)
            .build();

        //send the email
        match mailer.send(&message.unwrap()) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Could not send email: {e:?}");
                Err(MailSenderError::CouldntSendEmail.into())
            }
        }
    }

    pub fn new() -> MailSender {
        MailSender::default()
    }
}
