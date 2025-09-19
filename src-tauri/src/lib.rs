mod config;
mod mail_list_utils;
mod mail_sender;
mod ron_utils;

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

use crate::mail_list_utils::MailList;
use crate::mail_list_utils::Person;
use crate::mail_sender::MailSender;

use std::sync::Mutex;

use maud::{html, Markup};

struct AppState {
    mail: Mutex<MailSender>,
    mail_list: Mutex<MailList>,
}

#[tauri::command]
fn load_mechanics(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    let mail_list = app_state.mail_list.lock().unwrap();

    let markup: Markup = html! {
        @for i in 0..24 {
            @if let Some(mechanic) = mail_list.load_person(i){
                button.middle-button
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
                {(technic.name)}
            }
            @else{
                button.middle-button.placeholder{}
            }
        }
        button.middle-button{("ostatní...")}
    };

    markup.into_string()
}
#[tauri::command]

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
    let mail = app_state.mail.lock().unwrap();

    mail.send().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState {
                mail: MailSender::default().into(),
                mail_list: MailList::load().into(),
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            pick_file_handler,
            send_handler,
            load_mechanics,
            load_technics,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
