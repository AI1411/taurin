use i18nrs::yew::use_translation;
use yew::prelude::*;

#[function_component(LanguageSwitcher)]
pub fn language_switcher() -> Html {
    let (i18n, set_language) = use_translation();
    let current_lang = i18n.get_current_language();

    let on_switch_to_en = {
        let set_language = set_language.clone();
        Callback::from(move |_| {
            set_language.emit("en".to_string());
        })
    };

    let on_switch_to_ja = {
        let set_language = set_language.clone();
        Callback::from(move |_| {
            set_language.emit("ja".to_string());
        })
    };

    html! {
        <div class="language-switcher">
            <button
                class={classes!("lang-btn", (current_lang == "en").then_some("active"))}
                onclick={on_switch_to_en}
                title="English"
            >
                {"EN"}
            </button>
            <button
                class={classes!("lang-btn", (current_lang == "ja").then_some("active"))}
                onclick={on_switch_to_ja}
                title="Japanese"
            >
                {"JA"}
            </button>
        </div>
    }
}
