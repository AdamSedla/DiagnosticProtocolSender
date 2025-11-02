use serde::{Deserialize, Serialize};

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
    pub fn save_list(&mut self) {
        self.list.iter_mut().for_each(|person| {
            if person.as_ref().is_some_and(|person_unwrap| {
                person_unwrap.mail.is_empty() || person_unwrap.name.is_empty()
            }) {
                *person = None;
            }
        });

        let ron_string =
            ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default()).unwrap();

        std::fs::write("mail_list.ron", ron_string).unwrap();
    }

    pub fn load_list() -> MailList {
        let ron_string = std::fs::read_to_string("mail_list.ron").unwrap();
        ron::de::from_str(&ron_string).unwrap()
    }

    pub fn load_person(&self, id: usize) -> Option<Person> {
        self.list[id].clone()
    }

    pub fn save_person_name(&mut self, id: usize, name: String) {
        let mut person = match self.load_person(id) {
            Some(person) => person,
            None => Person {
                name: "".to_string(),
                mail: "".to_string(),
            },
        };

        person.name = name;

        self.list[id] = Some(person);
    }

    pub fn save_person_mail(&mut self, id: usize, mail: String) {
        let mut person = match self.load_person(id) {
            Some(person) => person,
            None => Person {
                name: "".to_string(),
                mail: "".to_string(),
            },
        };

        person.mail = mail;

        self.list[id] = Some(person);
    }
}
