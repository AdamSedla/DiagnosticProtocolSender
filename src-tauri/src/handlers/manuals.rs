use maud::{html, Markup};

//---------------------------

#[tauri::command]
pub fn open_manual() -> String {
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
pub fn close_manual() -> String {
    let markup: Markup = html! {
        div #manual-placeholder {}
    };

    markup.into_string()
}

#[tauri::command]
pub fn open_settings_manual() -> String {
    let markup: Markup = html! {
            div .overlay .most-top #overlay-manual{
                div .overlay-window{
                    button.close-button
                    hx-post="command:close_settings_manual"
                    hx-trigger="click"
                    hx-target="#overlay-manual"
                    hx-swap="outerHTML"
                    {("X")}
                    h1.overlay-title{("Návod k použití nastavení")}
                    ol.manual-text{
                        li{("Vyberte osobu ke změně nebo smazání údajů")}
                        ol{
                            li{("Přidání osoby - Přidejte jméno a E-mail vybrané osoby")}
                            li{("Úprava osoby - Upravte jméno nebo E-mail vybrané osoby")}
                            li{("Smazání osoby - Smažte jméno osoby")}
                        }
                        li{("Po dokončení změn")}
                        ol{
                            li{("Pro uložení změn - klikněte na \"uložit a zavřít\"")}
                            li{("Pro zrušení všech změn - klikněte na \"zavřít bez uložení\"")}
                        }
                    }
                }
            }
    };
    markup.into_string()
}

#[tauri::command]
pub fn close_settings_manual() -> String {
    let markup: Markup = html! {
        div #settings-manual-placeholder {}
    };

    markup.into_string()
}
