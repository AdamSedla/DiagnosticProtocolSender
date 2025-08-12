use std::error::Error;
use std::fmt::Display;
use std::fs;

use lettre::message::Mailbox;
use lettre::message::{Attachment, Body, MultiPart, header::ContentType};
use lettre::{Address, Message, SmtpTransport, Transport};

#[derive(Debug)]
pub enum MailSenderError {
    InvalidEmail,
    NotInDatabase,
    InvalidFilePath,
    NoReceivers,
    NoFile,
    CouldntSendEmail,
}

impl Error for MailSenderError {}

impl Display for MailSenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MailSenderError::InvalidEmail => write!(f, "invalid email"),
            MailSenderError::NotInDatabase => write!(f, "not in database"),
            MailSenderError::InvalidFilePath => write!(f, "invalid file path"),
            MailSenderError::NoReceivers => write!(f, "no receivers"),
            MailSenderError::NoFile => write!(f, "no file"),
            MailSenderError::CouldntSendEmail => write!(f, "couldn't send email"),
        }
    }
}

#[derive(Debug)]

pub struct Receiver {
    pub name: String,
    pub mail: Address,
}

pub struct MailSender {
    people: Vec<Receiver>,
    file: Body,
}

impl Default for MailSender {
    fn default() -> Self {
        MailSender {
            people: vec![],
            file: Body::new("".to_string()),
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

    pub fn add_file(&mut self, path: &str) -> Result<(), MailSenderError> {
        let pdf_file = fs::read(path);

        if pdf_file.is_err() {
            return Err(MailSenderError::InvalidFilePath);
        }

        self.file = Body::new(pdf_file.unwrap());

        Ok(())
    }

    pub fn send(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.people.is_empty() {
            return Err(Box::new(MailSenderError::NoReceivers));
        }
        if self.file.is_empty() {
            return Err(Box::new(MailSenderError::NoFile));
        }

        let mut message_builder = Message::builder();

        //sender
        message_builder = message_builder.from(Mailbox::new(
            Some("MAN Diag Postřižín".to_string()),
            "man.diag.postrizin@gmail.com".parse()?,
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
        message_builder = message_builder.subject("Diagnostický protokol z válcové zkušebny");

        //attachment
        let message = message_builder.multipart(MultiPart::mixed().singlepart(
            Attachment::new("protocol".to_string()).body(
                self.file.clone(),
                ContentType::parse("application/pdf").unwrap(),
            ),
        ));

        //get credentials
        let creds = todo!();

        // open a remote connection to gmail
        let mailer = SmtpTransport::relay("smtp.gmail.com")
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
