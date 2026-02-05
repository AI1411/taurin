use i18nrs::yew::use_translation;
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

#[derive(Clone, PartialEq, Copy)]
enum Mode {
    UnixToDatetime,
    DatetimeToUnix,
}

#[derive(Clone, PartialEq, Copy, Serialize, Deserialize)]
enum TimestampUnit {
    Seconds,
    Milliseconds,
}

#[derive(Clone, PartialEq, Copy, Serialize, Deserialize)]
enum TimezoneOption {
    Local,
    Utc,
}

#[derive(Serialize)]
struct UnixToDatetimeArgs {
    timestamp: i64,
    unit: TimestampUnit,
    timezone: TimezoneOption,
}

#[derive(Serialize)]
struct DatetimeToUnixArgs {
    datetime_str: String,
    timezone: TimezoneOption,
}

#[derive(Debug, Clone, Deserialize)]
struct UnixToDatetimeResult {
    success: bool,
    datetime: String,
    iso8601: String,
    date: String,
    time: String,
    day_of_week: String,
    relative_time: String,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct DatetimeToUnixResult {
    success: bool,
    unix_seconds: i64,
    unix_milliseconds: i64,
    error: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct CurrentUnixTimeResult {
    unix_seconds: i64,
    unix_milliseconds: i64,
    datetime: String,
    iso8601: String,
}

#[function_component(UnixTimeConverter)]
pub fn unix_time_converter() -> Html {
    let (i18n, _) = use_translation();
    let mode = use_state(|| Mode::UnixToDatetime);
    let input = use_state(String::new);
    let unit = use_state(|| TimestampUnit::Seconds);
    let timezone = use_state(|| TimezoneOption::Local);
    let is_processing = use_state(|| false);
    let error = use_state(|| Option::<String>::None);
    let copy_feedback = use_state(|| false);

    // Results for Unix to Datetime
    let datetime_result = use_state(|| Option::<UnixToDatetimeResult>::None);

    // Results for Datetime to Unix
    let unix_result = use_state(|| Option::<DatetimeToUnixResult>::None);

    // Current time display
    let current_time = use_state(|| Option::<CurrentUnixTimeResult>::None);

    // Fetch current time on mount and periodically
    {
        let current_time = current_time.clone();
        use_effect_with((), move |_| {
            let current_time = current_time.clone();

            let fetch_current_time = {
                let current_time = current_time.clone();
                move || {
                    let current_time = current_time.clone();
                    spawn_local(async move {
                        let result = invoke("get_current_unix_time_cmd", JsValue::NULL).await;
                        if let Ok(res) =
                            serde_wasm_bindgen::from_value::<CurrentUnixTimeResult>(result)
                        {
                            current_time.set(Some(res));
                        }
                    });
                }
            };

            // Initial fetch
            fetch_current_time();

            // Set up interval for updating current time
            let interval_handle = gloo_timers::callback::Interval::new(1000, move || {
                let current_time = current_time.clone();
                spawn_local(async move {
                    let result = invoke("get_current_unix_time_cmd", JsValue::NULL).await;
                    if let Ok(res) = serde_wasm_bindgen::from_value::<CurrentUnixTimeResult>(result)
                    {
                        current_time.set(Some(res));
                    }
                });
            });

            move || drop(interval_handle)
        });
    }

    let on_mode_change = {
        let mode = mode.clone();
        let input = input.clone();
        let datetime_result = datetime_result.clone();
        let unix_result = unix_result.clone();
        let error = error.clone();
        Callback::from(move |new_mode: Mode| {
            mode.set(new_mode);
            input.set(String::new());
            datetime_result.set(None);
            unix_result.set(None);
            error.set(None);
        })
    };

    let on_input_change = {
        let input = input.clone();
        Callback::from(move |e: InputEvent| {
            let target: web_sys::HtmlInputElement = e.target_unchecked_into();
            input.set(target.value());
        })
    };

    let on_change = {
        let input = input.clone();
        Callback::from(move |e: Event| {
            let target: web_sys::HtmlInputElement = e.target_unchecked_into();
            input.set(target.value());
        })
    };

    let on_unit_change = {
        let unit = unit.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            match select.value().as_str() {
                "seconds" => unit.set(TimestampUnit::Seconds),
                "milliseconds" => unit.set(TimestampUnit::Milliseconds),
                _ => {}
            }
        })
    };

    let on_timezone_change = {
        let timezone = timezone.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            match select.value().as_str() {
                "local" => timezone.set(TimezoneOption::Local),
                "utc" => timezone.set(TimezoneOption::Utc),
                _ => {}
            }
        })
    };

    let on_convert = {
        let mode = mode.clone();
        let input = input.clone();
        let unit = unit.clone();
        let timezone = timezone.clone();
        let is_processing = is_processing.clone();
        let error = error.clone();
        let datetime_result = datetime_result.clone();
        let unix_result = unix_result.clone();

        Callback::from(move |_| {
            let current_mode = *mode;
            let input_val = (*input).clone();
            let current_unit = *unit;
            let current_timezone = *timezone;
            let is_processing = is_processing.clone();
            let error = error.clone();
            let datetime_result = datetime_result.clone();
            let unix_result = unix_result.clone();

            if input_val.trim().is_empty() {
                return;
            }

            is_processing.set(true);

            spawn_local(async move {
                match current_mode {
                    Mode::UnixToDatetime => {
                        if let Ok(timestamp) = input_val.trim().parse::<i64>() {
                            let args = serde_wasm_bindgen::to_value(&UnixToDatetimeArgs {
                                timestamp,
                                unit: current_unit,
                                timezone: current_timezone,
                            })
                            .unwrap();
                            let result = invoke("unix_to_datetime_cmd", args).await;

                            if let Ok(res) =
                                serde_wasm_bindgen::from_value::<UnixToDatetimeResult>(result)
                            {
                                if res.success {
                                    datetime_result.set(Some(res));
                                    error.set(None);
                                } else {
                                    error.set(res.error);
                                }
                            }
                        } else {
                            error.set(Some("Invalid timestamp format".to_string()));
                        }
                    }
                    Mode::DatetimeToUnix => {
                        let args = serde_wasm_bindgen::to_value(&DatetimeToUnixArgs {
                            datetime_str: input_val,
                            timezone: current_timezone,
                        })
                        .unwrap();
                        let result = invoke("datetime_to_unix_cmd", args).await;

                        if let Ok(res) =
                            serde_wasm_bindgen::from_value::<DatetimeToUnixResult>(result)
                        {
                            if res.success {
                                unix_result.set(Some(res));
                                error.set(None);
                            } else {
                                error.set(res.error);
                            }
                        }
                    }
                }

                is_processing.set(false);
            });
        })
    };

    let on_copy = {
        let copy_feedback = copy_feedback.clone();
        Callback::from(move |text: String| {
            let copy_feedback = copy_feedback.clone();
            if let Some(win) = window() {
                let clipboard = win.navigator().clipboard();
                spawn_local(async move {
                    let _ = wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&text)).await;
                    copy_feedback.set(true);

                    let copy_feedback_reset = copy_feedback.clone();
                    gloo_timers::callback::Timeout::new(2000, move || {
                        copy_feedback_reset.set(false);
                    })
                    .forget();
                });
            }
        })
    };

    let on_use_current_time = {
        let input = input.clone();
        let current_time = current_time.clone();
        let unit = unit.clone();
        Callback::from(move |_| {
            if let Some(ct) = (*current_time).clone() {
                let timestamp = match *unit {
                    TimestampUnit::Seconds => ct.unix_seconds.to_string(),
                    TimestampUnit::Milliseconds => ct.unix_milliseconds.to_string(),
                };
                input.set(timestamp);
            }
        })
    };

    let on_clear = {
        let input = input.clone();
        let datetime_result = datetime_result.clone();
        let unix_result = unix_result.clone();
        let error = error.clone();
        Callback::from(move |_| {
            input.set(String::new());
            datetime_result.set(None);
            unix_result.set(None);
            error.set(None);
        })
    };

    html! {
        <div class="unix-time-converter">
            // Current time display
            if let Some(ct) = (*current_time).clone() {
                <div class="section current-time-section">
                    <div class="current-time-header">
                        <h3>{i18n.t("unix_time_converter.current_time")}</h3>
                    </div>
                    <div class="current-time-display">
                        <div class="time-item">
                            <span class="time-label">{i18n.t("unix_time_converter.unix_seconds")}</span>
                            <span class="time-value">{ct.unix_seconds.to_string()}</span>
                            <button
                                class="mini-copy-btn"
                                onclick={
                                    let on_copy = on_copy.clone();
                                    let value = ct.unix_seconds.to_string();
                                    Callback::from(move |_| on_copy.emit(value.clone()))
                                }
                            >
                                {i18n.t("common.copy")}
                            </button>
                        </div>
                        <div class="time-item">
                            <span class="time-label">{i18n.t("unix_time_converter.unix_milliseconds")}</span>
                            <span class="time-value">{ct.unix_milliseconds.to_string()}</span>
                            <button
                                class="mini-copy-btn"
                                onclick={
                                    let on_copy = on_copy.clone();
                                    let value = ct.unix_milliseconds.to_string();
                                    Callback::from(move |_| on_copy.emit(value.clone()))
                                }
                            >
                                {i18n.t("common.copy")}
                            </button>
                        </div>
                        <div class="time-item">
                            <span class="time-label">{i18n.t("unix_time_converter.local_time")}</span>
                            <span class="time-value">{ct.datetime.clone()}</span>
                        </div>
                    </div>
                </div>
            }

            // Mode selector
            <div class="section mode-section">
                <div class="mode-tabs">
                    <button
                        class={classes!("mode-tab", (*mode == Mode::UnixToDatetime).then_some("active"))}
                        onclick={
                            let on_mode_change = on_mode_change.clone();
                            Callback::from(move |_| on_mode_change.emit(Mode::UnixToDatetime))
                        }
                    >
                        {i18n.t("unix_time_converter.mode_unix_to_datetime")}
                    </button>
                    <button
                        class={classes!("mode-tab", (*mode == Mode::DatetimeToUnix).then_some("active"))}
                        onclick={
                            let on_mode_change = on_mode_change.clone();
                            Callback::from(move |_| on_mode_change.emit(Mode::DatetimeToUnix))
                        }
                    >
                        {i18n.t("unix_time_converter.mode_datetime_to_unix")}
                    </button>
                </div>
            </div>

            // Options section
            <div class="section options-section">
                <div class="options-row">
                    if *mode == Mode::UnixToDatetime {
                        <div class="option-group">
                            <label>{i18n.t("unix_time_converter.unit")}</label>
                            <select onchange={on_unit_change}>
                                <option value="seconds" selected={*unit == TimestampUnit::Seconds}>
                                    {i18n.t("unix_time_converter.unit_seconds")}
                                </option>
                                <option value="milliseconds" selected={*unit == TimestampUnit::Milliseconds}>
                                    {i18n.t("unix_time_converter.unit_milliseconds")}
                                </option>
                            </select>
                        </div>
                    }
                    <div class="option-group">
                        <label>{i18n.t("unix_time_converter.timezone")}</label>
                        <select onchange={on_timezone_change}>
                            <option value="local" selected={*timezone == TimezoneOption::Local}>
                                {i18n.t("unix_time_converter.timezone_local")}
                            </option>
                            <option value="utc" selected={*timezone == TimezoneOption::Utc}>
                                {i18n.t("unix_time_converter.timezone_utc")}
                            </option>
                        </select>
                    </div>
                </div>
            </div>

            // Input section
            <div class="section input-section">
                <div class="section-header">
                    <h3>
                        {match *mode {
                            Mode::UnixToDatetime => i18n.t("unix_time_converter.unix_input"),
                            Mode::DatetimeToUnix => i18n.t("unix_time_converter.datetime_input"),
                        }}
                    </h3>
                    <div class="input-actions">
                        if *mode == Mode::UnixToDatetime {
                            <button class="secondary-btn" onclick={on_use_current_time}>
                                {i18n.t("unix_time_converter.use_current")}
                            </button>
                        }
                        <button class="secondary-btn" onclick={on_clear}>
                            {i18n.t("common.clear")}
                        </button>
                    </div>
                </div>
                <input
                    type="text"
                    class="input-field"
                    placeholder={match *mode {
                        Mode::UnixToDatetime => i18n.t("unix_time_converter.unix_placeholder"),
                        Mode::DatetimeToUnix => i18n.t("unix_time_converter.datetime_placeholder"),
                    }}
                    value={(*input).clone()}
                    oninput={on_input_change}
                    onchange={on_change}
                />
                if *mode == Mode::DatetimeToUnix {
                    <div class="format-hint">
                        {i18n.t("unix_time_converter.format_hint")}
                    </div>
                }
            </div>

            // Convert button
            <div class="action-buttons">
                <button
                    class="primary-btn"
                    onclick={on_convert}
                    disabled={*is_processing || (*input).is_empty()}
                >
                    if *is_processing {
                        <span class="processing">
                            <span class="spinner"></span>
                            {i18n.t("common.processing")}
                        </span>
                    } else {
                        {i18n.t("common.convert")}
                    }
                </button>
            </div>

            // Error display
            if let Some(err) = (*error).clone() {
                <div class="section error-section">
                    <div class="error-message">
                        {"⚠ "}{err}
                    </div>
                </div>
            }

            // Results section - Unix to Datetime
            if *mode == Mode::UnixToDatetime {
                if let Some(result) = (*datetime_result).clone() {
                    <div class="section result-section">
                        <div class="section-header">
                            <h3>{i18n.t("common.result")}</h3>
                        </div>
                        <div class="result-grid">
                            <div class="result-item">
                                <span class="result-label">{i18n.t("unix_time_converter.datetime")}</span>
                                <div class="result-value-row">
                                    <span class="result-value">{result.datetime.clone()}</span>
                                    <button
                                        class={classes!("mini-copy-btn", (*copy_feedback).then_some("copied"))}
                                        onclick={
                                            let on_copy = on_copy.clone();
                                            let value = result.datetime.clone();
                                            Callback::from(move |_| on_copy.emit(value.clone()))
                                        }
                                    >
                                        if *copy_feedback {
                                            {"✓"}
                                        } else {
                                            {i18n.t("common.copy")}
                                        }
                                    </button>
                                </div>
                            </div>
                            <div class="result-item">
                                <span class="result-label">{"ISO 8601"}</span>
                                <div class="result-value-row">
                                    <span class="result-value">{result.iso8601.clone()}</span>
                                    <button
                                        class="mini-copy-btn"
                                        onclick={
                                            let on_copy = on_copy.clone();
                                            let value = result.iso8601.clone();
                                            Callback::from(move |_| on_copy.emit(value.clone()))
                                        }
                                    >
                                        {i18n.t("common.copy")}
                                    </button>
                                </div>
                            </div>
                            <div class="result-item">
                                <span class="result-label">{i18n.t("unix_time_converter.date")}</span>
                                <span class="result-value">{result.date}</span>
                            </div>
                            <div class="result-item">
                                <span class="result-label">{i18n.t("unix_time_converter.time")}</span>
                                <span class="result-value">{result.time}</span>
                            </div>
                            <div class="result-item">
                                <span class="result-label">{i18n.t("unix_time_converter.day_of_week")}</span>
                                <span class="result-value">{result.day_of_week}</span>
                            </div>
                            <div class="result-item">
                                <span class="result-label">{i18n.t("unix_time_converter.relative_time")}</span>
                                <span class="result-value relative">{result.relative_time}</span>
                            </div>
                        </div>
                    </div>
                }
            }

            // Results section - Datetime to Unix
            if *mode == Mode::DatetimeToUnix {
                if let Some(result) = (*unix_result).clone() {
                    <div class="section result-section">
                        <div class="section-header">
                            <h3>{i18n.t("common.result")}</h3>
                        </div>
                        <div class="result-grid">
                            <div class="result-item">
                                <span class="result-label">{i18n.t("unix_time_converter.unix_seconds")}</span>
                                <div class="result-value-row">
                                    <span class="result-value">{result.unix_seconds.to_string()}</span>
                                    <button
                                        class={classes!("mini-copy-btn", (*copy_feedback).then_some("copied"))}
                                        onclick={
                                            let on_copy = on_copy.clone();
                                            let value = result.unix_seconds.to_string();
                                            Callback::from(move |_| on_copy.emit(value.clone()))
                                        }
                                    >
                                        if *copy_feedback {
                                            {"✓"}
                                        } else {
                                            {i18n.t("common.copy")}
                                        }
                                    </button>
                                </div>
                            </div>
                            <div class="result-item">
                                <span class="result-label">{i18n.t("unix_time_converter.unix_milliseconds")}</span>
                                <div class="result-value-row">
                                    <span class="result-value">{result.unix_milliseconds.to_string()}</span>
                                    <button
                                        class="mini-copy-btn"
                                        onclick={
                                            let on_copy = on_copy.clone();
                                            let value = result.unix_milliseconds.to_string();
                                            Callback::from(move |_| on_copy.emit(value.clone()))
                                        }
                                    >
                                        {i18n.t("common.copy")}
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                }
            }
        </div>
    }
}
