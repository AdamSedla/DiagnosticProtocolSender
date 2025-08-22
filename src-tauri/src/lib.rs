mod config;
mod mail_sender;
mod ron_utils;

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;

use crate::mail_sender::MailSender;
use crate::mail_sender::Receiver;

use std::sync::Mutex;

struct AppState {
    mail: Mutex<MailSender>,
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
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            pick_file_handler,
            send_handler,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
