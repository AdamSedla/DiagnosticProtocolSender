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
    settings_current_person_id: Mutex<Option<usize>>,
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
fn open_manual() -> String {
    let markup: Markup = html! {
        div .overlay .most-top #overlay-manual{
            div .overlay-window{
                button.close-button
                hx-post="command:close_manual"
                hx-trigger="click"
                hx-target="#overlay-manual"
                hx-swap="outerHTML"
                {("X")}
                h1.overlay-title{("Návod k použití")}
                ol.manual-text{
                    li{("Vyberte přjemce (možné vybrat více)")}
                    ol{
                        li{("Kliknutím na jméno ve výběru")}
                        li{("Kliknutím na \"Ostatní...\"")}
                        ol{
                            li{("Kliknutím na \"přidat další E-mail\"")}
                            li{("Zadáním E-mailu do nově přidaného pole")}
                            li{("V případě potřeby lze pole smazat tlačítkem \"smazat\"")}
                            li{("Po zadání všech E-mailů můžete okno standardně zavřít křížkem")}
                        }
                    }
                    li{("Vyberte soubor k odeslání (možné vybrat více)")}
                    li{("Klikněte na odeslat")}
                }
            }
        }

    };

    markup.into_string()
}

#[tauri::command]
fn close_manual() -> String {
    let markup: Markup = html! {
        div #manual-placeholder {}
    };

    markup.into_string()
}

#[tauri::command]
fn open_feedback() -> String {
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
                button.feedback-send-button
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
fn close_feedback() -> String {
    let markup: Markup = html! {
        div #feedback-placeholder {}
    };

    markup.into_string()
}

#[tauri::command]
fn send_feedback(text: String) -> String {
    if mail_sender::MailSender::send_feedback(text).is_ok() {
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

#[tauri::command]
fn open_settings_password() -> String {
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
                button.password-check-button
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
fn close_settings_password() -> String {
    let markup: Markup = html! {
        div #settings-placeholder {}
    };

    markup.into_string()
}

#[tauri::command]
fn open_settings(app: tauri::AppHandle) -> String {
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
                hx-target="#app-body"
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
fn open_settings_config() -> String {
    todo!()
}

#[tauri::command]
fn close_settings_config() -> String {
    todo!()
}

#[tauri::command]
fn open_settings_manual() -> String {
    todo!()
}

#[tauri::command]
fn close_settings_manual() -> String {
    todo!()
}

#[tauri::command]
fn load_settings_mechanics(app: tauri::AppHandle) -> String {
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
fn load_settings_technics(app: tauri::AppHandle) -> String {
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
fn edit_person(id: String, app: tauri::AppHandle) -> String {
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
                hx-target="#app-body"
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
fn edit_person_name(app: tauri::AppHandle, id: String, text: String) {
    let id: usize = id.parse().unwrap();

    let app_state = app.state::<AppState>();

    app_state
        .mail_list
        .lock()
        .unwrap()
        .save_person_name(id, text);
}

#[tauri::command]
fn edit_person_mail(app: tauri::AppHandle, id: String, text: String) {
    let id: usize = id.parse().unwrap();

    let app_state = app.state::<AppState>();

    app_state
        .mail_list
        .lock()
        .unwrap()
        .save_person_mail(id, text);
}

#[tauri::command]
fn mark_person(id: String, app: tauri::AppHandle) -> String {
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
fn unmark_person(id: String, app: tauri::AppHandle) -> String {
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
fn save_and_close_settings(app: tauri::AppHandle) -> String {
    let app_state = app.state::<AppState>();

    app_state.mail_list.lock().unwrap().save_list();

    close_settings()
}

#[tauri::command]
fn close_settings() -> String {
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
                hx-post="command:pick_file_handler"
                hx-swap="outerHTML"
                {("výběr souboru")}
                input.truck
                type="image"
                src="src/assets/send_truck.svg"
                alt="truck-icon"
                hx-trigger="click"
                hx-post="command:send_handler"
                {}
            }
        }
    };
    markup.into_string()
}

#[tauri::command]
fn pick_file_handler(app: tauri::AppHandle) -> String {
    app.dialog().file().pick_files(move |file_path| {
        let app_state = app.state::<AppState>();

        app_state
            .mail
            .lock()
            .unwrap()
            .add_file(file_path.unwrap())
            .unwrap();
    });

    let markup: Markup = html! {
        button.choosen-file-picker
        hx-trigger="click"
        hx-post="command:pick_file_handler"
        hx-swap="outerHTML"
        {"soubor(y) vybrán(y)"}
    };

    markup.into_string()
}

#[tauri::command]
fn send_handler(app: tauri::AppHandle) {
    let app_state = app.state::<AppState>();
    let mut mail = app_state.mail.lock().unwrap();
    let mut other_mail_list = app_state.other_mail_list.lock().unwrap();

    let file_valid = mail.file_is_valid();
    let mail_list_not_empty = mail.person_list_is_valid() && other_mail_list.is_empty();
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
                settings_current_person_id: None.into(),
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
            remove_other_row,
            open_manual,
            close_manual,
            open_feedback,
            close_feedback,
            send_feedback,
            open_settings_password,
            close_settings_password,
            open_settings,
            close_settings,
            open_settings_config,
            close_settings_config,
            open_settings_manual,
            close_settings_manual,
            save_and_close_settings,
            load_settings_mechanics,
            load_settings_technics,
            edit_person,
            mark_person,
            unmark_person,
            edit_person_name,
            edit_person_mail
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
