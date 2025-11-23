
//external imports

use std::sync::Mutex;
use tauri::Manager;

//---------------------------

//import backend for AppState

mod backend{
    pub mod config;
    pub mod mail_list_utils;
    pub mod mail_sender;
    pub mod other_mail_utils;
}

use crate::backend::config::Config;
use crate::backend::mail_sender::MailSender;
use crate::backend::mail_list_utils::MailList;
use crate::backend::other_mail_utils::OtherMailList;

struct AppState {
    mail: Mutex<MailSender>,
    mail_list: Mutex<MailList>,
    other_mail_list: Mutex<OtherMailList>,
    settings_current_person_id: Mutex<Option<usize>>,
    config: Mutex<Config>,
}

//---------------------------

//import handlers from other files

mod handlers{
    pub mod app_settings;
    pub mod config_edit;
    pub mod feedback;
    pub mod mail_send;
    pub mod manuals;
    pub mod other_mail;
}

/* 
app_settings:
    - open_settings_password
    - close_settings_password
    - open_settings
    - open_discard_overlay
    - close_discard_overlay
    - discard_and_close_settings
    - save_and_close_settings
    - wrong_mail_warning
    - close_wrong_mail_warning
    - close_settings
    - load_settings_mechanics
    - load_settings_technics
    - edit_person
    - mark_person
    - unmark_person
    - edit_person_name
    - edit_person_mail
*/
use crate::handlers::app_settings::*;

//---------------------------

/* 
config edit
    - open_settings_config
    - save_and_close_settings_config
    - discard_and_close_settings_config
    - save_sender_name
    - save_sender_mail
    - save_sender_password
    - save_title
    - save_smtp_transport
    - save_feedback_mail
    - save_feedback_recepient
    - save_feedback_subject
    - save_settings_password
*/
use crate::handlers::config_edit::*;

//---------------------------

/* 
feedback
    - open_feedback
    - close_feedback
    - send_feedback

*/
use crate::handlers::feedback::*;

//---------------------------

/* 
mail_send
    - send
    - load_mechanics
    - load_technics
    - add_person
    - remove_person
    - pick_file
*/
use crate::handlers::mail_send::*;

//---------------------------

/* 
manuals:
    - open_manual
    - close_manual
    - open_settings_manual
    - close_settings_manual
*/
use crate::handlers::manuals::*;

//---------------------------

/* 
other_mail
    - open_other
    - close_other
    - add_other_mail_row
    - remove_other_row
    - edit_mail
*/
use crate::handlers::other_mail::*;

//---------------------------


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppState {
                mail: MailSender::default().into(),
                mail_list: MailList::load_list().into(),
                other_mail_list: OtherMailList::default().into(),
                settings_current_person_id: None.into(),
                config: Config::load_config().into(),
            });
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![ //seřadit
            pick_file,
            send,
            load_mechanics,
            load_technics,
            open_other,
            add_other_mail_row,
            close_other,
            add_person,
            remove_person,
            edit_mail,
            remove_other_row,
            open_manual,
            close_manual,
            open_feedback,
            close_feedback,
            send_feedback,
            open_settings_password,
            close_settings_password,
            open_settings,
            discard_and_close_settings,
            open_settings_config,
            discard_and_close_settings_config,
            open_settings_manual,
            close_settings_manual,
            save_and_close_settings,
            load_settings_mechanics,
            load_settings_technics,
            edit_person,
            mark_person,
            unmark_person,
            edit_person_name,
            edit_person_mail,
            save_and_close_settings_config,
            save_sender_name,
            save_sender_mail,
            save_sender_password,
            save_title,
            save_smtp_transport,
            save_feedback_mail,
            save_feedback_recepient,
            save_feedback_subject,
            save_settings_password,
            close_wrong_mail_warning,
            close_settings,
            open_discard_overlay,
            close_discard_overlay,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
