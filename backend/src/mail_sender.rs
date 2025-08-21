use std::fs;
use std::path::{Path, PathBuf};

use lettre::message::Mailbox;
use lettre::message::{Attachment, Body, MultiPart, header::ContentType};
use lettre::{Address, Message, SmtpTransport, Transport};

use crate::config::config;

use thiserror::Error;

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

#[derive(Debug)]

pub struct Receiver {
    pub name: String,
    pub mail: Address,
}

pub struct MailSender {
    people: Vec<Receiver>,
    file_path: Option<PathBuf>,
}

impl Default for MailSender {
    fn default() -> Self {
        MailSender {
            people: vec![],
            file_path: None,
        }
    }
}

impl MailSender {
    pub fn add_person(&mut self, person: Receiver) {
        self.people.push(person);
    }

    pub fn remove_person(&mut self, person: Receiver) -> Result<(), MailSenderError> {
        let wanted_mail = person.mail;

        let person_position = self.people.iter().position(|x| x.mail == wanted_mail);

        if person_position.is_none() {
            return Err(MailSenderError::NotInDatabase);
        }

        self.people.remove(person_position.unwrap());

        Ok(())
    }

    pub fn add_file(&mut self, path_string: &str) -> Result<(), MailSenderError> {
        let path: PathBuf = path_string.into();

        if !path.is_file() {
            return Err(MailSenderError::InvalidFilePath);
        }

        self.file_path = Some(path);

        Ok(())
    }

    pub fn send(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.people.is_empty() {
            return Err(Box::new(MailSenderError::NoReceivers));
        }
        if self.file_path.is_none() {
            return Err(Box::new(MailSenderError::NoFile));
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
        let file = fs::read(self.file_path.as_ref().unwrap())?;

        let message = message_builder.multipart(MultiPart::mixed().singlepart(
            Attachment::new(config.get_attachment_name().to_string()).body(
                Body::new(file),
                ContentType::parse("application/pdf").unwrap(),
            ),
        ));

        //get credentials
        let creds = config.get_credentials();

        // open a remote connection to gmail
        let mailer = SmtpTransport::relay(config.get_smtp_transport()) //X
            .unwrap()
            .credentials(creds)
            .build();

        //send the email
        match mailer.send(&message.unwrap()) {
            Ok(_) => Ok(()),
            Err(e) => {
                println!("Could not send email: {e:?}");
                Err(Box::new(MailSenderError::CouldntSendEmail))
            }
        }
    }

    pub fn new() -> MailSender {
        MailSender::default()
    }
}
