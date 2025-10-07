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
    pub fn save_list(&self) {
        let ron_string =
            ron::ser::to_string_pretty(&self.list, ron::ser::PrettyConfig::default()).unwrap();

        std::fs::write("mail_list.ron", ron_string).unwrap();
    }

    pub fn load_list() -> MailList {
        let ron_string = std::fs::read_to_string("mail_list.ron").unwrap();
        ron::de::from_str(&ron_string).unwrap()
    }

    pub fn load_person(&self, id: usize) -> Option<Person> {
        self.list[id].clone()
    }

    pub fn save_person(&mut self, id: usize, person: Person) {
        self.list[id] = Some(person);
    }

    pub fn delete_person(&mut self, id: usize) {
        self.list[id] = None;
    }
}
