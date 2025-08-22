use serde::{Deserialize, Serialize};

use crate::ron_utils::RonUtils;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub name: String,
    pub mail: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MailList {
    list: Vec<Option<Person>>, //0-23 - mechanic | 24-28 - technique
}

impl MailList {
    pub fn save(&self) {
        RonUtils::save_mail_list(self);
    }

    pub fn load() -> MailList {
        RonUtils::load_mail_list()
    }

    pub fn load_person(&self, id: usize) -> Option<Person> {
        self.list[id].clone()
    }

    pub fn save_person(&mut self, id: usize, person: Person) {
        self.list[id] = Some(person);
    }

    pub fn delete_position(&mut self, id: usize) {
        self.list[id] = None;
    }
}
