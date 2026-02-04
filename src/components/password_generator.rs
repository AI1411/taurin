use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GeneratorMode {
    Password,
    Passphrase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PasswordOptions {
    length: u32,
    include_lowercase: bool,
    include_uppercase: bool,
    include_digits: bool,
    include_symbols: bool,
    exclude_ambiguous: bool,
    custom_exclude: String,
    count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PassphraseOptions {
    word_count: u32,
    separator: String,
    capitalize: bool,
    include_number: bool,
    count: u32,
}

#[derive(Serialize)]
struct GeneratePasswordsArgs {
    options: PasswordOptions,
}

#[derive(Serialize)]
struct GeneratePassphrasesArgs {
    options: PassphraseOptions,
}

#[derive(Debug, Clone, Deserialize)]
struct PasswordStrength {
    score: u32,
    label: String,
    entropy: f64,
}

#[derive(Debug, Clone, Deserialize)]
struct GeneratedPassword {
    value: String,
    strength: PasswordStrength,
}

#[derive(Debug, Clone, Deserialize)]
struct PasswordGenerateResult {
    success: bool,
    passwords: Vec<GeneratedPassword>,
    #[allow(dead_code)]
    error: Option<String>,
}

#[derive(Clone, PartialEq)]
struct DisplayPassword {
    value: String,
    strength: u32,
    strength_label: String,
    entropy: f64,
    copied: bool,
}

#[function_component(PasswordGenerator)]
pub fn password_generator() -> Html {
    let mode = use_state(|| GeneratorMode::Password);
    let length = use_state(|| 16u32);
    let include_lowercase = use_state(|| true);
    let include_uppercase = use_state(|| true);
    let include_digits = use_state(|| true);
    let include_symbols = use_state(|| true);
    let exclude_ambiguous = use_state(|| false);
    let custom_exclude = use_state(String::new);
    let count = use_state(|| 1u32);

    let word_count = use_state(|| 4u32);
    let separator = use_state(|| "-".to_string());
    let capitalize = use_state(|| true);
    let include_number = use_state(|| true);

    let generated_passwords = use_state(Vec::<DisplayPassword>::new);
    let is_generating = use_state(|| false);
    let copy_all_feedback = use_state(|| false);

    let on_mode_change = {
        let mode = mode.clone();
        let generated_passwords = generated_passwords.clone();
        Callback::from(move |new_mode: GeneratorMode| {
            mode.set(new_mode);
            generated_passwords.set(vec![]);
        })
    };

    let on_length_change = {
        let length = length.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<u32>() {
                length.set(value.clamp(4, 128));
            }
        })
    };

    let on_count_change = {
        let count = count.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<u32>() {
                count.set(value.clamp(1, 100));
            }
        })
    };

    let on_custom_exclude_change = {
        let custom_exclude = custom_exclude.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            custom_exclude.set(input.value());
        })
    };

    let on_word_count_change = {
        let word_count = word_count.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Ok(value) = input.value().parse::<u32>() {
                word_count.set(value.clamp(2, 10));
            }
        })
    };

    let on_separator_change = {
        let separator = separator.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            separator.set(select.value());
        })
    };

    let on_generate = {
        let mode = mode.clone();
        let length = length.clone();
        let include_lowercase = include_lowercase.clone();
        let include_uppercase = include_uppercase.clone();
        let include_digits = include_digits.clone();
        let include_symbols = include_symbols.clone();
        let exclude_ambiguous = exclude_ambiguous.clone();
        let custom_exclude = custom_exclude.clone();
        let count = count.clone();
        let word_count = word_count.clone();
        let separator = separator.clone();
        let capitalize = capitalize.clone();
        let include_number = include_number.clone();
        let generated_passwords = generated_passwords.clone();
        let is_generating = is_generating.clone();

        Callback::from(move |_| {
            let mode_value = (*mode).clone();
            let generated_passwords = generated_passwords.clone();
            let is_generating = is_generating.clone();

            is_generating.set(true);

            match mode_value {
                GeneratorMode::Password => {
                    let options = PasswordOptions {
                        length: *length,
                        include_lowercase: *include_lowercase,
                        include_uppercase: *include_uppercase,
                        include_digits: *include_digits,
                        include_symbols: *include_symbols,
                        exclude_ambiguous: *exclude_ambiguous,
                        custom_exclude: (*custom_exclude).clone(),
                        count: *count,
                    };

                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&GeneratePasswordsArgs { options })
                            .unwrap();
                        let result = invoke("generate_passwords_cmd", args).await;

                        if let Ok(res) =
                            serde_wasm_bindgen::from_value::<PasswordGenerateResult>(result)
                        {
                            if res.success {
                                let passwords: Vec<DisplayPassword> = res
                                    .passwords
                                    .into_iter()
                                    .map(|p| DisplayPassword {
                                        value: p.value,
                                        strength: p.strength.score,
                                        strength_label: p.strength.label,
                                        entropy: p.strength.entropy,
                                        copied: false,
                                    })
                                    .collect();
                                generated_passwords.set(passwords);
                            }
                        }

                        is_generating.set(false);
                    });
                }
                GeneratorMode::Passphrase => {
                    let options = PassphraseOptions {
                        word_count: *word_count,
                        separator: (*separator).clone(),
                        capitalize: *capitalize,
                        include_number: *include_number,
                        count: *count,
                    };

                    spawn_local(async move {
                        let args =
                            serde_wasm_bindgen::to_value(&GeneratePassphrasesArgs { options })
                                .unwrap();
                        let result = invoke("generate_passphrases_cmd", args).await;

                        if let Ok(res) =
                            serde_wasm_bindgen::from_value::<PasswordGenerateResult>(result)
                        {
                            if res.success {
                                let passwords: Vec<DisplayPassword> = res
                                    .passwords
                                    .into_iter()
                                    .map(|p| DisplayPassword {
                                        value: p.value,
                                        strength: p.strength.score,
                                        strength_label: p.strength.label,
                                        entropy: p.strength.entropy,
                                        copied: false,
                                    })
                                    .collect();
                                generated_passwords.set(passwords);
                            }
                        }

                        is_generating.set(false);
                    });
                }
            }
        })
    };

    let on_copy_single = {
        let generated_passwords = generated_passwords.clone();
        Callback::from(move |index: usize| {
            let generated_passwords = generated_passwords.clone();
            if let Some(password) = (*generated_passwords).get(index) {
                let value = password.value.clone();
                if let Some(win) = window() {
                    let clipboard = win.navigator().clipboard();
                    let generated_passwords_inner = generated_passwords.clone();
                    spawn_local(async move {
                        let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&value))
                            .await;

                        let mut passwords = (*generated_passwords_inner).clone();
                        if let Some(p) = passwords.get_mut(index) {
                            p.copied = true;
                        }
                        generated_passwords_inner.set(passwords);

                        let generated_passwords_reset = generated_passwords_inner.clone();
                        gloo_timers::callback::Timeout::new(2000, move || {
                            let mut passwords = (*generated_passwords_reset).clone();
                            if let Some(p) = passwords.get_mut(index) {
                                p.copied = false;
                            }
                            generated_passwords_reset.set(passwords);
                        })
                        .forget();
                    });
                }
            }
        })
    };

    let on_copy_all = {
        let generated_passwords = generated_passwords.clone();
        let copy_all_feedback = copy_all_feedback.clone();
        Callback::from(move |_| {
            let passwords = (*generated_passwords).clone();
            let copy_all_feedback = copy_all_feedback.clone();
            if !passwords.is_empty() {
                let all_values: String = passwords
                    .iter()
                    .map(|p| p.value.clone())
                    .collect::<Vec<_>>()
                    .join("\n");

                if let Some(win) = window() {
                    let clipboard = win.navigator().clipboard();
                    spawn_local(async move {
                        let _ =
                            wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&all_values))
                                .await;
                        copy_all_feedback.set(true);

                        let copy_all_feedback_reset = copy_all_feedback.clone();
                        gloo_timers::callback::Timeout::new(2000, move || {
                            copy_all_feedback_reset.set(false);
                        })
                        .forget();
                    });
                }
            }
        })
    };

    let strength_class = |score: u32| -> &'static str {
        match score {
            1 => "strength-very-weak",
            2 => "strength-weak",
            3 => "strength-medium",
            4 => "strength-strong",
            _ => "strength-very-strong",
        }
    };

    html! {
        <div class="password-generator">
            // Mode Toggle
            <div class="section">
                <h3>{"生成モード"}</h3>
                <div class="mode-toggle">
                    <button
                        class={classes!("mode-btn", (*mode == GeneratorMode::Password).then_some("active"))}
                        onclick={{
                            let on_mode_change = on_mode_change.clone();
                            Callback::from(move |_| on_mode_change.emit(GeneratorMode::Password))
                        }}
                    >
                        {"パスワード"}
                    </button>
                    <button
                        class={classes!("mode-btn", (*mode == GeneratorMode::Passphrase).then_some("active"))}
                        onclick={{
                            let on_mode_change = on_mode_change.clone();
                            Callback::from(move |_| on_mode_change.emit(GeneratorMode::Passphrase))
                        }}
                    >
                        {"パスフレーズ"}
                    </button>
                </div>
            </div>

            // Password Options
            if *mode == GeneratorMode::Password {
                <div class="section password-options-section">
                    <h3>{"パスワード設定"}</h3>

                    <div class="password-options">
                        <div class="form-group">
                            <label>{"文字数"}</label>
                            <div class="length-slider">
                                <input
                                    type="range"
                                    min="4"
                                    max="128"
                                    value={length.to_string()}
                                    oninput={on_length_change.clone()}
                                />
                                <input
                                    type="number"
                                    class="form-input length-input"
                                    min="4"
                                    max="128"
                                    value={length.to_string()}
                                    oninput={on_length_change}
                                />
                            </div>
                        </div>

                        <div class="form-group">
                            <label>{"文字種"}</label>
                            <div class="char-type-options">
                                <label class="checkbox-option-inline">
                                    <input
                                        type="checkbox"
                                        checked={*include_lowercase}
                                        onchange={{
                                            let include_lowercase = include_lowercase.clone();
                                            Callback::from(move |_| include_lowercase.set(!*include_lowercase))
                                        }}
                                    />
                                    <span>{"小文字 (a-z)"}</span>
                                </label>
                                <label class="checkbox-option-inline">
                                    <input
                                        type="checkbox"
                                        checked={*include_uppercase}
                                        onchange={{
                                            let include_uppercase = include_uppercase.clone();
                                            Callback::from(move |_| include_uppercase.set(!*include_uppercase))
                                        }}
                                    />
                                    <span>{"大文字 (A-Z)"}</span>
                                </label>
                                <label class="checkbox-option-inline">
                                    <input
                                        type="checkbox"
                                        checked={*include_digits}
                                        onchange={{
                                            let include_digits = include_digits.clone();
                                            Callback::from(move |_| include_digits.set(!*include_digits))
                                        }}
                                    />
                                    <span>{"数字 (0-9)"}</span>
                                </label>
                                <label class="checkbox-option-inline">
                                    <input
                                        type="checkbox"
                                        checked={*include_symbols}
                                        onchange={{
                                            let include_symbols = include_symbols.clone();
                                            Callback::from(move |_| include_symbols.set(!*include_symbols))
                                        }}
                                    />
                                    <span>{"記号 (!@#$...)"}</span>
                                </label>
                            </div>
                        </div>

                        <div class="form-group">
                            <label class="checkbox-option-inline exclude-option">
                                <input
                                    type="checkbox"
                                    checked={*exclude_ambiguous}
                                    onchange={{
                                        let exclude_ambiguous = exclude_ambiguous.clone();
                                        Callback::from(move |_| exclude_ambiguous.set(!*exclude_ambiguous))
                                    }}
                                />
                                <span>{"紛らわしい文字を除外 (0, O, l, 1, I)"}</span>
                            </label>
                        </div>

                        <div class="form-group">
                            <label>{"追加除外文字"}</label>
                            <input
                                type="text"
                                class="form-input"
                                placeholder="除外する文字を入力..."
                                value={(*custom_exclude).clone()}
                                oninput={on_custom_exclude_change}
                            />
                        </div>

                        <div class="form-group">
                            <label>{"生成個数"}</label>
                            <input
                                type="number"
                                class="form-input"
                                min="1"
                                max="100"
                                value={count.to_string()}
                                oninput={on_count_change.clone()}
                            />
                        </div>
                    </div>
                </div>
            }

            // Passphrase Options
            if *mode == GeneratorMode::Passphrase {
                <div class="section passphrase-options-section">
                    <h3>{"パスフレーズ設定"}</h3>

                    <div class="passphrase-options">
                        <div class="form-group">
                            <label>{"単語数"}</label>
                            <div class="length-slider">
                                <input
                                    type="range"
                                    min="2"
                                    max="10"
                                    value={word_count.to_string()}
                                    oninput={on_word_count_change.clone()}
                                />
                                <input
                                    type="number"
                                    class="form-input length-input"
                                    min="2"
                                    max="10"
                                    value={word_count.to_string()}
                                    oninput={on_word_count_change}
                                />
                            </div>
                        </div>

                        <div class="form-group">
                            <label>{"区切り文字"}</label>
                            <select class="form-select" onchange={on_separator_change}>
                                <option value="-" selected={*separator == "-"}>{"ハイフン (-)"}</option>
                                <option value="_" selected={*separator == "_"}>{"アンダースコア (_)"}</option>
                                <option value="." selected={*separator == "."}>{"ドット (.)"}</option>
                                <option value=" " selected={*separator == " "}>{"スペース"}</option>
                                <option value="" selected={separator.is_empty()}>{"なし"}</option>
                            </select>
                        </div>

                        <div class="form-group">
                            <div class="passphrase-toggles">
                                <label class="checkbox-option-inline">
                                    <input
                                        type="checkbox"
                                        checked={*capitalize}
                                        onchange={{
                                            let capitalize = capitalize.clone();
                                            Callback::from(move |_| capitalize.set(!*capitalize))
                                        }}
                                    />
                                    <span>{"単語の先頭を大文字"}</span>
                                </label>
                                <label class="checkbox-option-inline">
                                    <input
                                        type="checkbox"
                                        checked={*include_number}
                                        onchange={{
                                            let include_number = include_number.clone();
                                            Callback::from(move |_| include_number.set(!*include_number))
                                        }}
                                    />
                                    <span>{"数字を追加"}</span>
                                </label>
                            </div>
                        </div>

                        <div class="form-group">
                            <label>{"生成個数"}</label>
                            <input
                                type="number"
                                class="form-input"
                                min="1"
                                max="100"
                                value={count.to_string()}
                                oninput={on_count_change}
                            />
                        </div>
                    </div>
                </div>
            }

            // Generate Button
            <button
                class="primary-btn generate-btn"
                onclick={on_generate}
                disabled={*is_generating}
            >
                if *is_generating {
                    <span class="processing">
                        <span class="spinner"></span>
                        {"生成中..."}
                    </span>
                } else {
                    if *mode == GeneratorMode::Password {
                        {"パスワードを生成"}
                    } else {
                        {"パスフレーズを生成"}
                    }
                }
            </button>

            // Generated Passwords
            if !generated_passwords.is_empty() {
                <div class="section password-results-section">
                    <div class="password-results-header">
                        <h3>{format!("生成結果 ({} 件)", generated_passwords.len())}</h3>
                        <button
                            class={classes!("secondary-btn", "copy-all-btn", (*copy_all_feedback).then_some("copied"))}
                            onclick={on_copy_all}
                        >
                            if *copy_all_feedback {
                                {"✓ コピー完了"}
                            } else {
                                {"すべてコピー"}
                            }
                        </button>
                    </div>
                    <div class="password-list">
                        { for (*generated_passwords).iter().enumerate().map(|(index, password)| {
                            let on_copy = {
                                let on_copy_single = on_copy_single.clone();
                                Callback::from(move |_| on_copy_single.emit(index))
                            };
                            html! {
                                <div class="password-item">
                                    <div class="password-content">
                                        <code class="password-value">{&password.value}</code>
                                        <div class="password-meta">
                                            <span class={classes!("strength-badge", strength_class(password.strength))}>
                                                {&password.strength_label}
                                            </span>
                                            <span class="entropy-value">
                                                {format!("エントロピー: {:.1} bits", password.entropy)}
                                            </span>
                                        </div>
                                    </div>
                                    <button
                                        class={classes!("copy-btn", password.copied.then_some("copied"))}
                                        onclick={on_copy}
                                    >
                                        if password.copied {
                                            {"✓"}
                                        } else {
                                            {"📋"}
                                        }
                                    </button>
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
        </div>
    }
}
