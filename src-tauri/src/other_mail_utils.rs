use crate::mail_list_utils::Person;

use maud::{html, Markup};

#[derive(Default, Debug)]
pub struct OtherMailList {
    list: Vec<Person>,
}

impl OtherMailList {
    pub fn size(&self) -> usize {
        self.list.len()
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

        /*
                        <div class="other-mail-button-row">
                        <input class="other-mail-input-field" type="text" placeholder="Zadejte prosím E-mail">
                        <button class="remove-button">Odstranit</button>
                        </div>
        */
    }
}
