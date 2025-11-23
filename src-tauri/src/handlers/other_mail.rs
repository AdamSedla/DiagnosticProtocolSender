use maud::{html, Markup};
use tauri::Manager;

use crate::AppState;
//---------------------------

#[tauri::command]
pub fn open_other(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    let markup: Markup = html! {
        div #overlay-other .overlay
        {
            div.overlay-window
            {
                button.close-button
                hx-post="command:close_other"
                hx-trigger="click"
                hx-target="#overlay-other"
                hx-swap="outerHTML"
                {("X")}
                h1.overlay-title{("zadejte prosím E-mailové adresy")}
                div.other-mail-buttons #other-mail-buttons
                {(app_state.other_mail_list.lock().unwrap().render_input_fields())}
                div.bottom-button-row{
                    button.add-button
                    hx-post="command:add_other_mail_row"
                    hx-trigger="click"
                    hx-target="#other-mail-list-placeholder"
                    hx-swap="outerHTML"
                    {("přidat další E-mail")}
                }
            }
        }
    };
    markup.into_string()
}

#[tauri::command]
pub fn add_other_mail_row(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    let index = app_state.other_mail_list.lock().unwrap().size();

    let markup: Markup = html! {
        div.other-mail-button-row{
            input.other-mail-input-field
            type="text"
            hx-post="command:edit_mail"
            name="text"
            hx-trigger="change"
            placeholder="Zadejte prosím E-mail"
            hx-vals={(format!(r#""index": {index}"#))}
            {}
            button.remove-button
            hx-post="command:remove_other_row"
            hx-trigger="click"
            hx-target="#other-mail-buttons"
            hx-swap="innerHTML"
            hx-vals={(format!(r#""index": {index}"#))}
            {("odstranit")}
        }

        div #other-mail-list-placeholder {}
    };

    app_state.other_mail_list.lock().unwrap().increment_size();
    app_state.other_mail_list.lock().unwrap().add_person();

    markup.into_string()
}

#[tauri::command]
pub fn edit_mail(app: tauri::AppHandle, index: String, text: String) {
    let app_state = app.state::<AppState>();

    let index: usize = index.parse().unwrap();

    app_state
        .other_mail_list
        .lock()
        .unwrap()
        .edit_person(&text, index);
}

#[tauri::command]
pub fn remove_other_row(app: tauri::AppHandle, index: String) -> String {
    let app_state: tauri::State<'_, AppState> = app.state::<AppState>();
    let index: usize = index.parse().unwrap();

    app_state
        .other_mail_list
        .lock()
        .unwrap()
        .remove_person(index);

    let markup: Markup = app_state
        .other_mail_list
        .lock()
        .unwrap()
        .render_input_fields();

    markup.into_string()
}

#[tauri::command]
pub fn close_other(app: tauri::AppHandle) -> String {
    let markup: Markup = html! {
        div #overlay-other-placeholder {}
    };

    let app_state = app.state::<AppState>();

    app_state
        .other_mail_list
        .lock()
        .unwrap()
        .remove_empty_persons();

    markup.into_string()
}
