mod config;
mod mail_list_utils;
mod mail_sender;
mod other_mail_utils;

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

use crate::mail_sender::MailSender;
use crate::{mail_list_utils::MailList, other_mail_utils::OtherMailList};

use std::sync::Mutex;

use maud::{html, Markup};

struct AppState {
    mail: Mutex<MailSender>,
    mail_list: Mutex<MailList>,
    other_mail_list: Mutex<OtherMailList>,
}

#[tauri::command]
fn load_mechanics(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    let mail_list = app_state.mail_list.lock().unwrap();

    let markup: Markup = html! {
        @for i in 0..24 {
            @if let Some(mechanic) = mail_list.load_person(i){
                button.middle-button
                hx-trigger="click"
                hx-post="command:add_person"
                hx-swap="outerHTML"
                hx-vals={(format!(r#""id": {i}"#))}
                {(mechanic.name)}
            }
            @else{
                button.middle-button.placeholder{}
            }
        }
    };

    markup.into_string()
}

#[tauri::command]
fn load_technics(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    let mail_list = app_state.mail_list.lock().unwrap();

    let markup: Markup = html! {
        @for i in 24..29 {
            @if let Some(technic) = mail_list.load_person(i){
                button.middle-button
                hx-trigger="click"
                hx-post="command:add_person"
                hx-swap="outerHTML"
                hx-vals={(format!(r#""id": {i}"#))}
                {(technic.name)}
            }
            @else{
                button.middle-button.placeholder{}
            }
        }
        button.middle-button
        hx-post="command:open_other"
        hx-trigger="click"
        hx-target="#overlay-other-placeholder"
        hx-swap="outerHTML"
        {("ostatní...")}
    };

    markup.into_string()
}

#[tauri::command]
fn add_person(id: String, app: tauri::AppHandle) -> String {
    let id: usize = id.parse().unwrap();
    let app_state = app.state::<AppState>();

    let person = app_state.mail_list.lock().unwrap().load_person(id).unwrap();

    app_state.mail.lock().unwrap().add_person(person.clone());

    let markup: Markup = html! {
        button.middle-button.clicked
            hx-trigger="click"
            hx-post="command:remove_person"
            hx-swap="outerHTML"
            hx-vals={(format!(r#""id": {id}"#))}
        {(person.name)}
    };

    markup.into_string()
}

#[tauri::command]
fn remove_person(id: String, app: tauri::AppHandle) -> String {
    let id: usize = id.parse().unwrap();
    let app_state = app.state::<AppState>();

    let person = app_state.mail_list.lock().unwrap().load_person(id).unwrap();

    app_state.mail.lock().unwrap().remove_person(person.clone());

    let markup: Markup = html! {
        button.middle-button
            hx-trigger="click"
            hx-post="command:add_person"
            hx-swap="outerHTML"
            hx-vals={(format!(r#""id": {id}"#))}
        {(person.name)}
    };

    markup.into_string()
}

#[tauri::command]
fn open_other(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    let markup: Markup = html! {
        div #overlay-other .overlay-other
        {
            div.other-mail-window
            {
                button.close-button
                hx-post="command:close_other"
                hx-trigger="click"
                hx-target="#overlay-other"
                hx-swap="outerHTML"
                {("X")}
                h1.other-mail-title{("zadejte prosím E-mailové adresy")}
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
fn add_other_mail_row(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    let index = app_state.other_mail_list.lock().unwrap().size();

    let markup: Markup = html! {
        div.other-mail-button-row{
            input.other-mail-input-field
            type="text"
            hx-post="command:edit_mail"
            name="text"
            hx-trigger="change"
            placeholder="zadejte prosím E-mail"
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
fn edit_mail(app: tauri::AppHandle, index: String, text: String) {
    let app_state = app.state::<AppState>();

    let index: usize = index.parse().unwrap();

    app_state
        .other_mail_list
        .lock()
        .unwrap()
        .edit_person(&text, index);
}

#[tauri::command]
fn remove_other_row(app: tauri::AppHandle, index: String) -> String {
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
fn close_other(app: tauri::AppHandle) -> String {
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

#[tauri::command]
fn pick_file_handler(app: tauri::AppHandle) {
    app.dialog().file().pick_file(move |file_path| {
        let app_state = app.state::<AppState>();

        app_state
            .mail
            .lock()
            .unwrap()
            .add_file(file_path.unwrap())
            .unwrap();
    });
}

#[tauri::command]
fn send_handler(app: tauri::AppHandle) {
    let app_state = app.state::<AppState>();
    let mut mail = app_state.mail.lock().unwrap();
    let mut other_mail_list = app_state.other_mail_list.lock().unwrap();

    let file_valid = mail.file_is_valid();
    let mail_list_not_empty = (mail.person_list_is_valid() && other_mail_list.is_empty());
    let other_mail_list_valid = other_mail_list.is_valid();

    //valid check
    if !(file_valid && ((mail_list_not_empty) || other_mail_list_valid)) {
        return;
    }

    mail.send(other_mail_list.export_other_mail_list()).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState {
                mail: MailSender::default().into(),
                mail_list: MailList::load_list().into(),
                other_mail_list: OtherMailList::default().into(),
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            pick_file_handler,
            send_handler,
            load_mechanics,
            load_technics,
            open_other,
            add_other_mail_row,
            close_other,
            add_person,
            remove_person,
            edit_mail,
            remove_other_row
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
