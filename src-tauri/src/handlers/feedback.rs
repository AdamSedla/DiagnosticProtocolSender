use maud::{html, Markup};
use tauri::Manager;

use crate::AppState;
use crate::backend::mail_sender;
//---------------------------

#[tauri::command]
pub fn open_feedback() -> String {
    let markup: Markup = html! {
        div .overlay .most-top #overlay-feedback{
            div .overlay-window{
                button.close-button
                hx-post="command:close_feedback"
                hx-trigger="click"
                hx-target="#overlay-feedback"
                hx-swap="outerHTML"
                {("X")}
                h1.overlay-title{("hlášení chyb a nápady na vylepšení")}
                textarea.feedback-input
                name="text"
                placeholder="Zadejte prosím zprávu pro vývojáře"
                {}
                button.feedback-send-button.save
                hx-post="command:send_feedback"
                hx-trigger="click"
                hx-include="[name='text']"
                hx-swap="outerHTML"
                {("odeslat")}
            }
        }
    };

    markup.into_string()
}

#[tauri::command]
pub fn close_feedback() -> String {
    let markup: Markup = html! {
        div #feedback-placeholder {}
    };

    markup.into_string()
}

#[tauri::command]
pub fn send_feedback(text: String, app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();
    let config = app_state.config.lock().unwrap().clone();

    if mail_sender::MailSender::send_feedback(text, config).is_ok() {
        let markup: Markup = html! {
            h1.feedback-send-message{("Zpětná vazba byla odeslána, děkujeme!")}
        };
        return markup.into_string();
    } else {
        let markup: Markup = html! {
            h1.feedback-send-message{"Nepodařilo se odeslat zpětnou vazbu."
            br;
            "Kontaktujte prosím administrátora!"}
        };
        return markup.into_string();
    }
}
