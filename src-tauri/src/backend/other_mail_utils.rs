use lettre::Address;
use maud::{html, Markup};

use crate::backend::mail_list_utils::Person;

//---------------------------

#[derive(Default, Debug)]
pub struct OtherMailList {
    list: Vec<Option<Person>>,
    size: usize,
}

impl OtherMailList {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn render_input_fields(&self) -> Markup {
        let markup: Markup = html! {
            @for (index, person) in (self.list.iter().enumerate()) {
                @if person.is_some(){
                       div.other-mail-button-row{
                            input.other-mail-input-field
                            type="text"
                            hx-post="command:edit_mail"
                            name="text"
                            hx-trigger="change"
                            hx-vals={(format!(r#""index": {index}"#))}
                            placeholder="Zadejte prosím E-mail"
                            value=(person.as_ref().unwrap().mail)
                            {}
                            button.remove-button
                            hx-post="command:remove_other_row"
                            hx-trigger="click"
                            hx-target="#other-mail-buttons"
                            hx-swap="innerHTML"
                            hx-vals={(format!(r#""index": {index}"#))}
                            {("odstranit")}
                       }
                }
            }
            div #other-mail-list-placeholder {}
        };
        markup
    }

    pub fn add_person(&mut self) {
        self.list.push(Some(Person {
            name: "".to_string(),
            mail: "".to_string(),
        }));
    }

    pub fn edit_person(&mut self, mail: &str, index: usize) {
        self.list[index] = Some(Person {
            name: mail.to_string(),
            mail: mail.to_string(),
        });
    }

    pub fn remove_person(&mut self, index: usize) {
        self.list[index] = None;
    }

    pub fn increment_size(&mut self) {
        self.size += 1;
    }

    pub fn remove_empty_persons(&mut self) {
        self.list.iter_mut().for_each(|person| {
            if person
                .as_ref()
                .is_some_and(|person_unwrap| person_unwrap.mail.is_empty())
            {
                *person = None;
            }
        });
    }

    pub fn export_other_mail_list(&mut self) -> Vec<Person> {
        let mut final_vec: Vec<Person> = vec![];

        self.list.iter().for_each(|person| {
            if person.is_some() {
                final_vec.push(person.clone().unwrap());
            }
        });

        final_vec
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }

    pub fn is_valid(&self) -> bool {
        self.list
            .iter()
            .filter_map(|person| person.as_ref())
            .all(|person| person.mail.parse::<Address>().is_ok())
    }

    pub fn clear(&mut self) {
        self.list.clear();
        self.size = 0;
    }
}
