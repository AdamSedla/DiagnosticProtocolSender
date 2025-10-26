use std::usize;

use crate::mail_list_utils::Person;

use maud::{html, Markup};

#[derive(Default, Debug)]
pub struct OtherMailList {
    list: Vec<Person>,
    size: usize,
}

impl OtherMailList {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn render_input_fields(&self) -> Markup {
        let markup: Markup;
        if self.list.is_empty() {
            markup = html! {
                div #other-mail-list-placeholder {}
            }
        } else {
            markup = html! {
                div.other-mail-button-row{
                    input.other-mail-input-field
                    type="text"
                    placeholder="zadejte prosím E-mail"
                    {}
                    button.remove-button{("odstranit")}
                }
            };
        }

        markup
    }

    pub fn edit_person(&mut self, mail: &str, index: usize) {
        println!("Index: {index} : size: {}", self.list.len());
    }

    pub fn increment_size(&mut self) {
        self.size += 1;
    }
}
