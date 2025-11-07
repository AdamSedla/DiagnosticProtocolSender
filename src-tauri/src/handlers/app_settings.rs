use maud::{html, Markup};
use tauri::Manager;

use crate::AppState;
use crate::backend::mail_list_utils;

//---------------------------

#[tauri::command]
pub fn open_settings_password() -> String {
    let markup: Markup = html! {
        div .overlay .most-top #overlay-password{
            div .overlay-window{
                button.close-button
                hx-post="command:close_settings_password"
                hx-trigger="click"
                hx-target="#overlay-password"
                hx-swap="outerHTML"
                {("X")}
                h1.password-title{("Zadejte prosím heslo pro vstup do nastavení")}
                input.password-input
                placeholder="Heslo"
                {}
                button.password-check-button.save
                hx-post="command:open_settings"
                hx-trigger="click"
                hx-target="#app-body"
                hx-swap="outerHTML"
                {("ověřit")}
            }
        }
    };

    markup.into_string()
}

#[tauri::command]
pub fn close_settings_password() -> String {
    let markup: Markup = html! {
        div #settings-placeholder {}
    };

    markup.into_string()
}

#[tauri::command]
pub fn open_settings(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    app_state.mail.lock().unwrap().clear();
    app_state.other_mail_list.lock().unwrap().clear();

    let markup: Markup = html! {
        body #app-body {
            div.top-bar{
                div.top-button-bar{
                    button.top-bar-button
                    hx-post="command:open_settings_config"
                    hx-trigger="click"
                    hx-target="#settings-config-placeholder"
                    hx-swap="outerHTML"
                    {("config")}
                    button.top-bar-button
                    hx-post="command:open_feedback"
                    hx-trigger="click"
                    hx-target="#feedback-placeholder"
                    hx-swap="outerHTML"
                    {("hlášení chyb a nápady na vylepšení")}
                    button.top-bar-button
                    hx-post="command:open_settings_manual"
                    hx-trigger="click"
                    hx-target="#settings-manual-placeholder"
                    hx-swap="outerHTML"
                    {("návod k použití")}
                }
                img.man-logo
                src="src/assets/man_logo_batch.svg"
                alt="man-logo"
                {}

            }
            div.center-buttons{
                div.mechanic-buttons
                hx-trigger="load delay:1ms"
                hx-swap="innerHTML"
                hx-post="command:load_settings_mechanics"
                {}
                div.right-buttons
                hx-trigger="load delay:1ms"
                hx-swap="innerHTML"
                hx-post="command:load_settings_technics"
                {}
            }
            div #feedback-placeholder{}
            div #settings-manual-placeholder{}
            div #settings-config-placeholder{}
            div #valid-mail-placeholder{}
            div.bottom-bar #bottom-bar{
            div.bottom-part-settings-names{
                h1.settings-bottom-text{("Vyberte prosím osobu pro úpravu údajů")}
            }
            div.bottom-part-settings-names{
            }
            div.bottom-part-settings-buttons{
                button.settings-bottom-button.save
                hx-post="command:save_and_close_settings"
                hx-trigger="click"
                hx-target="#valid-mail-placeholder"
                hx-swap="outerHTML"
                {("uložit a zavřít")}
                button.settings-bottom-button.close{("zavřít bez uložení")}
            }
            }
        }
    };

    markup.into_string()
}

#[tauri::command]
pub fn save_and_close_settings(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    let mail_list_save = app_state.mail_list.lock().unwrap().save_list();

    if mail_list_save.is_err(){
        wrong_mail_warning(mail_list_save.unwrap_err())
    }
    else {
        html!{
            div
            hx-trigger="load delay:1ms"
            hw-swap="outerHTML"
            hx-post="command:close_settings"
            hx-target="#app-body"
            {}
        }.into_string()
    }
}

pub fn wrong_mail_warning(name_list: Vec<String>) -> String {
    html!{
        div .overlay #wrong-mail-warning-overlay{
            div .overlay-window{
                button.close-button
                hx-post="command:close_wrong_mail_warning"
                hx-trigger="click"
                hx-target="#wrong-mail-warning-overlay"
                hx-swap="outerHTML"
                {("X")}
                h1.overlay-title{("Následující osoby mají neplatný E-mail")}
                div.mail-warning-rows-section{
                    @for (name) in (name_list){
                        h2.mail-warning-row{(name)};
                    }



                }
                h1.overlay-title{("Upravte nebo smažte je")}

            }
        }
    }.into_string()
 }

#[tauri::command]
pub fn close_wrong_mail_warning() -> String{
    html! {
        div #valid-mail-placeholder{}
    }.into_string()
}

#[tauri::command]
pub fn discard_and_close_settings(app: tauri::AppHandle) -> String {
    //todo: discard

    //close_settings()
    
    todo!()
}

#[tauri::command]
pub fn close_settings() -> String {
    let markup: Markup = html! {
        body #app-body {
            div.top-bar{
                div.top-button-bar{
                    button.top-bar-button
                    hx-post="command:open_settings_password"
                    hx-trigger="click"
                    hx-target="#settings-placeholder"
                    hx-swap="outerHTML"
                    {("nastavení")}
                    button.top-bar-button
                    hx-post="command:open_feedback"
                    hx-trigger="click"
                    hx-target="#feedback-placeholder"
                    hx-swap="outerHTML"
                    {("hlášení chyb a nápady na vylepšení")}
                    button.top-bar-button
                    hx-post="command:open_manual"
                    hx-trigger="click"
                    hx-target="#manual-placeholder"
                    hx-swap="outerHTML"
                    {("návod k použití")}
                }
                img.man-logo
                src="src/assets/man_logo_batch.svg"
                alt="man-logo"
                {}
            }
            div.center-buttons{
                div.mechanic-buttons
                hx-trigger="load delay:1ms"
                hx-swap="innerHTML"
                hx-post="command:load_mechanics"
                {}
                div.right-buttons
                hx-trigger="load delay:1ms"
                hx-swap="innerHTML"
                hx-post="command:load_technics"
                {}
            }
            div #overlay-other-placeholder{}
            div #feedback-placeholder{}
            div #manual-placeholder{}
            div #settings-placeholder{}
            div.bottom-bar{
                button.file-picker
                hx-trigger="click"
                hx-post="command:pick_file"
                hx-swap="outerHTML"
                {("výběr souboru")}
                input.truck
                type="image"
                src="src/assets/send_truck.svg"
                alt="truck-icon"
                hx-trigger="click"
                hx-post="command:send"
                {}
            }
        }
    };
    markup.into_string()
}

#[tauri::command]
pub fn load_settings_mechanics(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    let mail_list = app_state.mail_list.lock().unwrap();

    let markup: Markup = html! {
        @for i in 0..24 {
            @if let Some(mechanic) = mail_list.load_person(i){
                button.middle-button
                id=(format!("id-{}", i))
                hx-trigger="click"
                hx-post="command:edit_person"
                hx-swap="outerHTML"
                hx-target="#bottom-bar"
                hx-vals={(format!(r#""id": {i}"#))}
                {(mechanic.name)}
            }
            @else{
                button.middle-button
                id=(format!("id-{}", i))
                hx-trigger="click"
                hx-post="command:edit_person"
                hx-swap="outerHTML"
                hx-target="#bottom-bar"
                hx-vals={(format!(r#""id": {i}"#))}
                {}
            }
        }
    };

    markup.into_string()
}

#[tauri::command]
pub fn load_settings_technics(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    let mail_list = app_state.mail_list.lock().unwrap();

    let markup: Markup = html! {
    @for i in 24..29 {
        @if let Some(technic) = mail_list.load_person(i){
            button.middle-button
            id=(format!("id-{}", i))
            hx-trigger="click"
            hx-post="command:edit_person"
            hx-swap="outerHTML"
            hx-target="#bottom-bar"
            hx-vals={(format!(r#""id": {i}"#))}
            {(technic.name)}
        }
        @else{
            button.middle-button
            id=(format!("id-{}", i))
            hx-trigger="click"
            hx-post="command:edit_person"
            hx-swap="outerHTML"
            hx-target="#bottom-bar"
            hx-vals={(format!(r#""id": {i}"#))}
            {}
        }
    }
    button.middle-button.placeholder{}
    };

    markup.into_string()
}

#[tauri::command]
pub fn edit_person(id: String, app: tauri::AppHandle) -> String {
    let id: usize = id.parse().unwrap();

    let app_state = app.state::<AppState>();

    let mail_list = app_state.mail_list.lock().unwrap();

    let person = match mail_list.load_person(id) {
        Some(person) => person,
        None => mail_list_utils::Person {
            name: "".to_string(),
            mail: "".to_string(),
        },
    };

    let markup: Markup = html! {
        div.bottom-bar #bottom-bar {
            div.bottom-part-settings-names{
                h1.settings-bottom-text{("jméno")}
                input.settings-bottom-input
                type="text"
                hx-post="command:edit_person_name"
                name="text"
                hx-trigger="change"
                hx-vals={(format!(r#""id": {id}"#))}
                value=(person.name)
                {}
            }
            div.bottom-part-settings-names{
                h1.settings-bottom-text{("e-mail")}
                input.settings-bottom-input
                type="text"
                hx-post="command:edit_person_mail"
                name="text"
                hx-trigger="change"
                hx-vals={(format!(r#""id": {id}"#))}
                value=(person.mail)
                {}
            }
            div.bottom-part-settings-buttons{
                button.settings-bottom-button.save
                hx-post="command:save_and_close_settings"
                hx-trigger="click"
                hx-target="#valid-mail-placeholder"
                hx-swap="outerHTML"
                {("uložit a zavřít")}
                button.settings-bottom-button.close{("zavřít bez uložení")}
            }
        }
        div
        hx-trigger="load delay:1ms"
        hx-swap="outerHTML"
        hx-target=(format!("#id-{}", id))
        hx-vals={(format!(r#""id": {id}"#))}
        hx-post="command:mark_person"
        {}
        @if let Some(id) = *app_state.settings_current_person_id.lock().unwrap() {
            div
            hx-trigger="load delay:1ms"
            hx-swap="outerHTML"
            hx-target=(format!("#id-{}", id))
            hx-vals={(format!(r#""id": {id}"#))}
            hx-post="command:unmark_person"
            {}
        }
    };

    *app_state.settings_current_person_id.lock().unwrap() = Some(id);

    markup.into_string()
}

#[tauri::command]
pub fn mark_person(id: String, app: tauri::AppHandle) -> String {
    let id: usize = id.parse().unwrap();

    let app_state = app.state::<AppState>();

    let mail_list = app_state.mail_list.lock().unwrap();

    let person = match mail_list.load_person(id) {
        Some(person) => person,
        None => mail_list_utils::Person {
            name: "".to_string(),
            mail: "".to_string(),
        },
    };

    let markup: Markup = html! {
        button.middle-button.clicked
        id=(format!("id-{}", id))
        {(person.name)}
    };

    markup.into_string()
}

#[tauri::command]
pub fn unmark_person(id: String, app: tauri::AppHandle) -> String {
    let id: usize = id.parse().unwrap();

    let app_state = app.state::<AppState>();

    let mail_list = app_state.mail_list.lock().unwrap();

    let person = match mail_list.load_person(id) {
        Some(person) => person,
        None => mail_list_utils::Person {
            name: "".to_string(),
            mail: "".to_string(),
        },
    };

    let markup: Markup = html! {
        button.middle-button
        id=(format!("id-{}", id))
        hx-trigger="click"
        hx-post="command:edit_person"
        hx-swap="outerHTML"
        hx-target="#bottom-bar"
        hx-vals={(format!(r#""id": {id}"#))}
        {(person.name)}
    };

    markup.into_string()
}

#[tauri::command]
pub fn edit_person_name(app: tauri::AppHandle, id: String, text: String) {
    let id: usize = id.parse().unwrap();

    let app_state = app.state::<AppState>();

    app_state
        .mail_list
        .lock()
        .unwrap()
        .save_person_name(id, text);
}

#[tauri::command]
pub fn edit_person_mail(app: tauri::AppHandle, id: String, text: String) {
    let id: usize = id.parse().unwrap();

    let app_state = app.state::<AppState>();

    app_state
        .mail_list
        .lock()
        .unwrap()
        .save_person_mail(id, text);
}
