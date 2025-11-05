use maud::{html, Markup};
use tauri::Manager;

use crate::AppState;
use crate::backend::config::Config;

//---------------------------

#[tauri::command]
pub fn open_settings_config(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();
    let config = app_state.config.lock().unwrap().clone();

    let markup: Markup = html! {
        div #overlay-settings-config .overlay{
            div.overlay-window{
                button.close-button
                hx-post="command:discard_and_close_settings_config"
                hx-trigger="click"
                hx-target="#overlay-settings-config"
                hx-swap="outerHTML"
                {("X")}
                h1.overlay-title{("úprava konfiguračního souboru")}
                div.config-row-section{
                    div.config-row{
                        h1.config-row-title
                        {("Jméno odesilatele:")}
                        input.config-row-input-field
                        type="text"
                        hx-post="command:save_sender_name"
                        hx-trigger="change"
                        name="text"
                        value=(config.sender_name())
                        {}
                    }
                    div.config-row{
                        h1.config-row-title
                        {("E-mail odesilatele:")}
                        input.config-row-input-field
                        type="text"
                        hx-post="command:save_sender_mail"
                        hx-trigger="change"
                        name="text"
                        value=(config.sender_mail())
                        {}
                    }
                    div.config-row{
                        h1.config-row-title
                        {("heslo odesilatele:")}
                        input.config-row-input-field
                        type="text"
                        hx-post="command:save_sender_password"
                        hx-trigger="change"
                        name="text"
                        value=(config.sender_password())
                        {}
                    }
                    div.config-row{
                        h1.config-row-title
                        {("předmět E-mailu:")}
                        input.config-row-input-field
                        type="text"
                        hx-post="command:save_title"
                        hx-trigger="change"
                        name="text"
                        value=(config.title())
                        {}
                    }
                    div.config-row{
                        h1.config-row-title
                        {("smtp transport:")}
                        input.config-row-input-field
                        type="text"
                        hx-post="command:save_smtp_transport"
                        hx-trigger="change"
                        name="text"
                        value=(config.smtp_transport())
                        {}
                    }
                    div.config-row{
                        h1.config-row-title
                        {("feedback E-mail:")}
                        input.config-row-input-field
                        type="text"
                        hx-post="command:save_feedback_mail"
                        hx-trigger="change"
                        name="text"
                        value=(config.feedback_mail())
                        {}
                    }
                    div.config-row{
                        h1.config-row-title
                        {("feedback příjemce:")}
                        input.config-row-input-field
                        type="text"
                        hx-post="command:save_feedback_recepient"
                        hx-trigger="change"
                        name="text"
                        value=(config.feedback_recepient())
                        {}
                    }
                    div.config-row{
                        h1.config-row-title
                        {("feedback předmět:")}
                        input.config-row-input-field
                        type="text"
                        hx-post="command:save_feedback_subject"
                        hx-trigger="change"
                        name="text"
                        value=(config.feedback_subject())
                        {}
                    }
                    div.config-row{
                        h1.config-row-title
                        {("heslo nastavení:")}
                        input.config-row-input-field
                        type="text"
                        hx-post="command:save_settings_password"
                        hx-trigger="change"
                        name="text"
                        value=(config.settings_password())
                        {}
                    }
                }
                div.bottom-button-row{
                    button.save-config.save
                    hx-post="command:save_and_close_settings_config"
                    hx-trigger="click"
                    hx-target="#overlay-settings-config"
                    hx-swap="outerHTML"
                    {("uložit a zavřít")}
                }
            }
        }
    };

    markup.into_string()
}

#[tauri::command]
pub fn save_sender_name(app: tauri::AppHandle, text: String) {
    let app_state = app.state::<AppState>();

    app_state.config.lock().unwrap().save_sender_name(text);
}

#[tauri::command]
pub fn save_sender_mail(app: tauri::AppHandle, text: String) {
    let app_state = app.state::<AppState>();

    app_state.config.lock().unwrap().save_sender_mail(text);
}

#[tauri::command]
pub fn save_sender_password(app: tauri::AppHandle, text: String) {
    let app_state = app.state::<AppState>();

    app_state.config.lock().unwrap().save_sender_password(text);
}

#[tauri::command]
pub fn save_title(app: tauri::AppHandle, text: String) {
    let app_state = app.state::<AppState>();

    app_state.config.lock().unwrap().save_title(text);
}

#[tauri::command]
pub fn save_smtp_transport(app: tauri::AppHandle, text: String) {
    let app_state = app.state::<AppState>();

    app_state.config.lock().unwrap().save_smtp_transport(text);
}

#[tauri::command]
pub fn save_feedback_mail(app: tauri::AppHandle, text: String) {
    let app_state = app.state::<AppState>();

    app_state.config.lock().unwrap().save_feedback_mail(text);
}

#[tauri::command]
pub fn save_feedback_recepient(app: tauri::AppHandle, text: String) {
    let app_state = app.state::<AppState>();

    app_state
        .config
        .lock()
        .unwrap()
        .save_feedback_recepient(text);
}

#[tauri::command]
pub fn save_feedback_subject(app: tauri::AppHandle, text: String) {
    let app_state = app.state::<AppState>();

    app_state.config.lock().unwrap().save_feedback_subject(text);
}

#[tauri::command]
pub fn save_settings_password(app: tauri::AppHandle, text: String) {
    let app_state = app.state::<AppState>();

    app_state
        .config
        .lock()
        .unwrap()
        .save_settings_password(text);
}

#[tauri::command]
pub fn save_and_close_settings_config(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    app_state.config.lock().unwrap().save_config();

    close_settings_config()
}

#[tauri::command]
pub fn discard_and_close_settings_config(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    *app_state.config.lock().unwrap() = Config::load_config();

    close_settings_config()
}

pub fn close_settings_config() -> String {
    let markup: Markup = html! {
        div #settings-config-placeholder {}
    };

    markup.into_string()
}
