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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UnitCategory {
    Length,
    Weight,
    DataSize,
    Temperature,
    Time,
    Area,
    Volume,
}

impl UnitCategory {
    fn translation_key(&self) -> &'static str {
        match self {
            UnitCategory::Length => "unit_converter.category_length",
            UnitCategory::Weight => "unit_converter.category_weight",
            UnitCategory::DataSize => "unit_converter.category_data_size",
            UnitCategory::Temperature => "unit_converter.category_temperature",
            UnitCategory::Time => "unit_converter.category_time",
            UnitCategory::Area => "unit_converter.category_area",
            UnitCategory::Volume => "unit_converter.category_volume",
        }
    }

    fn icon(&self) -> &'static str {
        match self {
            UnitCategory::Length => "📏",
            UnitCategory::Weight => "⚖️",
            UnitCategory::DataSize => "💾",
            UnitCategory::Temperature => "🌡️",
            UnitCategory::Time => "⏱️",
            UnitCategory::Area => "📐",
            UnitCategory::Volume => "🧊",
        }
    }

    fn all() -> Vec<UnitCategory> {
        vec![
            UnitCategory::Length,
            UnitCategory::Weight,
            UnitCategory::DataSize,
            UnitCategory::Temperature,
            UnitCategory::Time,
            UnitCategory::Area,
            UnitCategory::Volume,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LengthUnit {
    Meter,
    Centimeter,
    Millimeter,
    Kilometer,
    Inch,
    Feet,
    Yard,
    Mile,
}

impl LengthUnit {
    #[allow(dead_code)]
    fn translation_key(&self) -> &'static str {
        match self {
            LengthUnit::Meter => "unit_converter.length_meter",
            LengthUnit::Centimeter => "unit_converter.length_centimeter",
            LengthUnit::Millimeter => "unit_converter.length_millimeter",
            LengthUnit::Kilometer => "unit_converter.length_kilometer",
            LengthUnit::Inch => "unit_converter.length_inch",
            LengthUnit::Feet => "unit_converter.length_feet",
            LengthUnit::Yard => "unit_converter.length_yard",
            LengthUnit::Mile => "unit_converter.length_mile",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            LengthUnit::Meter => "m",
            LengthUnit::Centimeter => "cm",
            LengthUnit::Millimeter => "mm",
            LengthUnit::Kilometer => "km",
            LengthUnit::Inch => "in",
            LengthUnit::Feet => "ft",
            LengthUnit::Yard => "yd",
            LengthUnit::Mile => "mi",
        }
    }

    fn all() -> Vec<LengthUnit> {
        vec![
            LengthUnit::Meter,
            LengthUnit::Centimeter,
            LengthUnit::Millimeter,
            LengthUnit::Kilometer,
            LengthUnit::Inch,
            LengthUnit::Feet,
            LengthUnit::Yard,
            LengthUnit::Mile,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WeightUnit {
    Kilogram,
    Gram,
    Milligram,
    Pound,
    Ounce,
    Ton,
}

impl WeightUnit {
    #[allow(dead_code)]
    fn translation_key(&self) -> &'static str {
        match self {
            WeightUnit::Kilogram => "unit_converter.weight_kilogram",
            WeightUnit::Gram => "unit_converter.weight_gram",
            WeightUnit::Milligram => "unit_converter.weight_milligram",
            WeightUnit::Pound => "unit_converter.weight_pound",
            WeightUnit::Ounce => "unit_converter.weight_ounce",
            WeightUnit::Ton => "unit_converter.weight_ton",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            WeightUnit::Kilogram => "kg",
            WeightUnit::Gram => "g",
            WeightUnit::Milligram => "mg",
            WeightUnit::Pound => "lb",
            WeightUnit::Ounce => "oz",
            WeightUnit::Ton => "t",
        }
    }

    fn all() -> Vec<WeightUnit> {
        vec![
            WeightUnit::Kilogram,
            WeightUnit::Gram,
            WeightUnit::Milligram,
            WeightUnit::Pound,
            WeightUnit::Ounce,
            WeightUnit::Ton,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DataSizeUnit {
    Byte,
    Kilobyte,
    Megabyte,
    Gigabyte,
    Terabyte,
    Petabyte,
    Kibibyte,
    Mebibyte,
    Gibibyte,
    Tebibyte,
}

impl DataSizeUnit {
    #[allow(dead_code)]
    fn translation_key(&self) -> &'static str {
        match self {
            DataSizeUnit::Byte => "unit_converter.data_byte",
            DataSizeUnit::Kilobyte => "unit_converter.data_kilobyte",
            DataSizeUnit::Megabyte => "unit_converter.data_megabyte",
            DataSizeUnit::Gigabyte => "unit_converter.data_gigabyte",
            DataSizeUnit::Terabyte => "unit_converter.data_terabyte",
            DataSizeUnit::Petabyte => "unit_converter.data_petabyte",
            DataSizeUnit::Kibibyte => "unit_converter.data_kibibyte",
            DataSizeUnit::Mebibyte => "unit_converter.data_mebibyte",
            DataSizeUnit::Gibibyte => "unit_converter.data_gibibyte",
            DataSizeUnit::Tebibyte => "unit_converter.data_tebibyte",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            DataSizeUnit::Byte => "B",
            DataSizeUnit::Kilobyte => "KB",
            DataSizeUnit::Megabyte => "MB",
            DataSizeUnit::Gigabyte => "GB",
            DataSizeUnit::Terabyte => "TB",
            DataSizeUnit::Petabyte => "PB",
            DataSizeUnit::Kibibyte => "KiB",
            DataSizeUnit::Mebibyte => "MiB",
            DataSizeUnit::Gibibyte => "GiB",
            DataSizeUnit::Tebibyte => "TiB",
        }
    }

    fn all() -> Vec<DataSizeUnit> {
        vec![
            DataSizeUnit::Byte,
            DataSizeUnit::Kilobyte,
            DataSizeUnit::Megabyte,
            DataSizeUnit::Gigabyte,
            DataSizeUnit::Terabyte,
            DataSizeUnit::Petabyte,
            DataSizeUnit::Kibibyte,
            DataSizeUnit::Mebibyte,
            DataSizeUnit::Gibibyte,
            DataSizeUnit::Tebibyte,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TemperatureUnit {
    #[allow(dead_code)]
    fn translation_key(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "unit_converter.temp_celsius",
            TemperatureUnit::Fahrenheit => "unit_converter.temp_fahrenheit",
            TemperatureUnit::Kelvin => "unit_converter.temp_kelvin",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "°C",
            TemperatureUnit::Fahrenheit => "°F",
            TemperatureUnit::Kelvin => "K",
        }
    }

    fn all() -> Vec<TemperatureUnit> {
        vec![
            TemperatureUnit::Celsius,
            TemperatureUnit::Fahrenheit,
            TemperatureUnit::Kelvin,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

impl TimeUnit {
    #[allow(dead_code)]
    fn translation_key(&self) -> &'static str {
        match self {
            TimeUnit::Second => "unit_converter.time_second",
            TimeUnit::Minute => "unit_converter.time_minute",
            TimeUnit::Hour => "unit_converter.time_hour",
            TimeUnit::Day => "unit_converter.time_day",
            TimeUnit::Week => "unit_converter.time_week",
            TimeUnit::Month => "unit_converter.time_month",
            TimeUnit::Year => "unit_converter.time_year",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            TimeUnit::Second => "s",
            TimeUnit::Minute => "min",
            TimeUnit::Hour => "h",
            TimeUnit::Day => "day",
            TimeUnit::Week => "week",
            TimeUnit::Month => "month",
            TimeUnit::Year => "year",
        }
    }

    fn all() -> Vec<TimeUnit> {
        vec![
            TimeUnit::Second,
            TimeUnit::Minute,
            TimeUnit::Hour,
            TimeUnit::Day,
            TimeUnit::Week,
            TimeUnit::Month,
            TimeUnit::Year,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AreaUnit {
    SquareMeter,
    SquareKilometer,
    SquareCentimeter,
    SquareFeet,
    SquareInch,
    Hectare,
    Acre,
    Tsubo,
}

impl AreaUnit {
    #[allow(dead_code)]
    fn translation_key(&self) -> &'static str {
        match self {
            AreaUnit::SquareMeter => "unit_converter.area_sqmeter",
            AreaUnit::SquareKilometer => "unit_converter.area_sqkilometer",
            AreaUnit::SquareCentimeter => "unit_converter.area_sqcentimeter",
            AreaUnit::SquareFeet => "unit_converter.area_sqfeet",
            AreaUnit::SquareInch => "unit_converter.area_sqinch",
            AreaUnit::Hectare => "unit_converter.area_hectare",
            AreaUnit::Acre => "unit_converter.area_acre",
            AreaUnit::Tsubo => "unit_converter.area_tsubo",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            AreaUnit::SquareMeter => "m²",
            AreaUnit::SquareKilometer => "km²",
            AreaUnit::SquareCentimeter => "cm²",
            AreaUnit::SquareFeet => "ft²",
            AreaUnit::SquareInch => "in²",
            AreaUnit::Hectare => "ha",
            AreaUnit::Acre => "ac",
            AreaUnit::Tsubo => "tsubo",
        }
    }

    fn all() -> Vec<AreaUnit> {
        vec![
            AreaUnit::SquareMeter,
            AreaUnit::SquareKilometer,
            AreaUnit::SquareCentimeter,
            AreaUnit::SquareFeet,
            AreaUnit::SquareInch,
            AreaUnit::Hectare,
            AreaUnit::Acre,
            AreaUnit::Tsubo,
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VolumeUnit {
    Liter,
    Milliliter,
    CubicMeter,
    CubicCentimeter,
    Gallon,
    Quart,
    Pint,
    Cup,
}

impl VolumeUnit {
    #[allow(dead_code)]
    fn translation_key(&self) -> &'static str {
        match self {
            VolumeUnit::Liter => "unit_converter.volume_liter",
            VolumeUnit::Milliliter => "unit_converter.volume_milliliter",
            VolumeUnit::CubicMeter => "unit_converter.volume_cubicmeter",
            VolumeUnit::CubicCentimeter => "unit_converter.volume_cubiccentimeter",
            VolumeUnit::Gallon => "unit_converter.volume_gallon",
            VolumeUnit::Quart => "unit_converter.volume_quart",
            VolumeUnit::Pint => "unit_converter.volume_pint",
            VolumeUnit::Cup => "unit_converter.volume_cup",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            VolumeUnit::Liter => "L",
            VolumeUnit::Milliliter => "mL",
            VolumeUnit::CubicMeter => "m³",
            VolumeUnit::CubicCentimeter => "cm³",
            VolumeUnit::Gallon => "gal",
            VolumeUnit::Quart => "qt",
            VolumeUnit::Pint => "pt",
            VolumeUnit::Cup => "cup",
        }
    }

    fn all() -> Vec<VolumeUnit> {
        vec![
            VolumeUnit::Liter,
            VolumeUnit::Milliliter,
            VolumeUnit::CubicMeter,
            VolumeUnit::CubicCentimeter,
            VolumeUnit::Gallon,
            VolumeUnit::Quart,
            VolumeUnit::Pint,
            VolumeUnit::Cup,
        ]
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConvertLengthArgs {
    value: f64,
    from: LengthUnit,
    to: LengthUnit,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConvertWeightArgs {
    value: f64,
    from: WeightUnit,
    to: WeightUnit,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConvertDataSizeArgs {
    value: f64,
    from: DataSizeUnit,
    to: DataSizeUnit,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConvertTemperatureArgs {
    value: f64,
    from: TemperatureUnit,
    to: TemperatureUnit,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConvertTimeArgs {
    value: f64,
    from: TimeUnit,
    to: TimeUnit,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConvertAreaArgs {
    value: f64,
    from: AreaUnit,
    to: AreaUnit,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ConvertVolumeArgs {
    value: f64,
    from: VolumeUnit,
    to: VolumeUnit,
}

#[derive(Debug, Clone, Deserialize)]
struct ConversionResult {
    success: bool,
    #[allow(dead_code)]
    result: f64,
    formatted: String,
    #[allow(dead_code)]
    error: Option<String>,
}

#[derive(Clone, PartialEq)]
struct HistoryEntry {
    category: UnitCategory,
    from_value: String,
    from_unit: String,
    to_value: String,
    to_unit: String,
}

#[function_component(UnitConverter)]
pub fn unit_converter() -> Html {
    let (i18n, _) = use_translation();
    let category = use_state(|| UnitCategory::Length);
    let input_value = use_state(String::new);
    let result_value = use_state(String::new);
    let is_converting = use_state(|| false);
    let history = use_state(Vec::<HistoryEntry>::new);
    let copied = use_state(|| false);

    // Unit states for each category
    let length_from = use_state(|| LengthUnit::Meter);
    let length_to = use_state(|| LengthUnit::Centimeter);
    let weight_from = use_state(|| WeightUnit::Kilogram);
    let weight_to = use_state(|| WeightUnit::Gram);
    let data_from = use_state(|| DataSizeUnit::Gigabyte);
    let data_to = use_state(|| DataSizeUnit::Megabyte);
    let temp_from = use_state(|| TemperatureUnit::Celsius);
    let temp_to = use_state(|| TemperatureUnit::Fahrenheit);
    let time_from = use_state(|| TimeUnit::Hour);
    let time_to = use_state(|| TimeUnit::Minute);
    let area_from = use_state(|| AreaUnit::SquareMeter);
    let area_to = use_state(|| AreaUnit::Tsubo);
    let volume_from = use_state(|| VolumeUnit::Liter);
    let volume_to = use_state(|| VolumeUnit::Milliliter);

    let on_category_change = {
        let category = category.clone();
        let result_value = result_value.clone();
        Callback::from(move |cat: UnitCategory| {
            category.set(cat);
            result_value.set(String::new());
        })
    };

    let on_input_change = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let on_swap_units = {
        let category = category.clone();
        let length_from = length_from.clone();
        let length_to = length_to.clone();
        let weight_from = weight_from.clone();
        let weight_to = weight_to.clone();
        let data_from = data_from.clone();
        let data_to = data_to.clone();
        let temp_from = temp_from.clone();
        let temp_to = temp_to.clone();
        let time_from = time_from.clone();
        let time_to = time_to.clone();
        let area_from = area_from.clone();
        let area_to = area_to.clone();
        let volume_from = volume_from.clone();
        let volume_to = volume_to.clone();
        let result_value = result_value.clone();

        Callback::from(move |_| {
            result_value.set(String::new());
            match *category {
                UnitCategory::Length => {
                    let from = (*length_from).clone();
                    let to = (*length_to).clone();
                    length_from.set(to);
                    length_to.set(from);
                }
                UnitCategory::Weight => {
                    let from = (*weight_from).clone();
                    let to = (*weight_to).clone();
                    weight_from.set(to);
                    weight_to.set(from);
                }
                UnitCategory::DataSize => {
                    let from = (*data_from).clone();
                    let to = (*data_to).clone();
                    data_from.set(to);
                    data_to.set(from);
                }
                UnitCategory::Temperature => {
                    let from = (*temp_from).clone();
                    let to = (*temp_to).clone();
                    temp_from.set(to);
                    temp_to.set(from);
                }
                UnitCategory::Time => {
                    let from = (*time_from).clone();
                    let to = (*time_to).clone();
                    time_from.set(to);
                    time_to.set(from);
                }
                UnitCategory::Area => {
                    let from = (*area_from).clone();
                    let to = (*area_to).clone();
                    area_from.set(to);
                    area_to.set(from);
                }
                UnitCategory::Volume => {
                    let from = (*volume_from).clone();
                    let to = (*volume_to).clone();
                    volume_from.set(to);
                    volume_to.set(from);
                }
            }
        })
    };

    let on_convert = {
        let category = category.clone();
        let input_value = input_value.clone();
        let result_value = result_value.clone();
        let is_converting = is_converting.clone();
        let history = history.clone();
        let length_from = length_from.clone();
        let length_to = length_to.clone();
        let weight_from = weight_from.clone();
        let weight_to = weight_to.clone();
        let data_from = data_from.clone();
        let data_to = data_to.clone();
        let temp_from = temp_from.clone();
        let temp_to = temp_to.clone();
        let time_from = time_from.clone();
        let time_to = time_to.clone();
        let area_from = area_from.clone();
        let area_to = area_to.clone();
        let volume_from = volume_from.clone();
        let volume_to = volume_to.clone();

        Callback::from(move |_| {
            let value = match (*input_value).parse::<f64>() {
                Ok(v) => v,
                Err(_) => return,
            };

            let cat = *category;
            let result_value = result_value.clone();
            let is_converting = is_converting.clone();
            let history = history.clone();
            let input_str = (*input_value).clone();

            is_converting.set(true);

            match cat {
                UnitCategory::Length => {
                    let from = (*length_from).clone();
                    let to = (*length_to).clone();
                    let from_label = from.label().to_string();
                    let to_label = to.label().to_string();
                    spawn_local(async move {
                        let args =
                            serde_wasm_bindgen::to_value(&ConvertLengthArgs { value, from, to })
                                .unwrap();
                        let result = invoke("convert_length_cmd", args).await;
                        if let Ok(res) = serde_wasm_bindgen::from_value::<ConversionResult>(result)
                        {
                            if res.success {
                                result_value.set(res.formatted.clone());
                                let mut h = (*history).clone();
                                h.insert(
                                    0,
                                    HistoryEntry {
                                        category: cat,
                                        from_value: input_str,
                                        from_unit: from_label,
                                        to_value: res.formatted,
                                        to_unit: to_label,
                                    },
                                );
                                if h.len() > 10 {
                                    h.pop();
                                }
                                history.set(h);
                            }
                        }
                        is_converting.set(false);
                    });
                }
                UnitCategory::Weight => {
                    let from = (*weight_from).clone();
                    let to = (*weight_to).clone();
                    let from_label = from.label().to_string();
                    let to_label = to.label().to_string();
                    spawn_local(async move {
                        let args =
                            serde_wasm_bindgen::to_value(&ConvertWeightArgs { value, from, to })
                                .unwrap();
                        let result = invoke("convert_weight_cmd", args).await;
                        if let Ok(res) = serde_wasm_bindgen::from_value::<ConversionResult>(result)
                        {
                            if res.success {
                                result_value.set(res.formatted.clone());
                                let mut h = (*history).clone();
                                h.insert(
                                    0,
                                    HistoryEntry {
                                        category: cat,
                                        from_value: input_str,
                                        from_unit: from_label,
                                        to_value: res.formatted,
                                        to_unit: to_label,
                                    },
                                );
                                if h.len() > 10 {
                                    h.pop();
                                }
                                history.set(h);
                            }
                        }
                        is_converting.set(false);
                    });
                }
                UnitCategory::DataSize => {
                    let from = (*data_from).clone();
                    let to = (*data_to).clone();
                    let from_label = from.label().to_string();
                    let to_label = to.label().to_string();
                    spawn_local(async move {
                        let args =
                            serde_wasm_bindgen::to_value(&ConvertDataSizeArgs { value, from, to })
                                .unwrap();
                        let result = invoke("convert_data_size_cmd", args).await;
                        if let Ok(res) = serde_wasm_bindgen::from_value::<ConversionResult>(result)
                        {
                            if res.success {
                                result_value.set(res.formatted.clone());
                                let mut h = (*history).clone();
                                h.insert(
                                    0,
                                    HistoryEntry {
                                        category: cat,
                                        from_value: input_str,
                                        from_unit: from_label,
                                        to_value: res.formatted,
                                        to_unit: to_label,
                                    },
                                );
                                if h.len() > 10 {
                                    h.pop();
                                }
                                history.set(h);
                            }
                        }
                        is_converting.set(false);
                    });
                }
                UnitCategory::Temperature => {
                    let from = (*temp_from).clone();
                    let to = (*temp_to).clone();
                    let from_label = from.label().to_string();
                    let to_label = to.label().to_string();
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&ConvertTemperatureArgs {
                            value,
                            from,
                            to,
                        })
                        .unwrap();
                        let result = invoke("convert_temperature_cmd", args).await;
                        if let Ok(res) = serde_wasm_bindgen::from_value::<ConversionResult>(result)
                        {
                            if res.success {
                                result_value.set(res.formatted.clone());
                                let mut h = (*history).clone();
                                h.insert(
                                    0,
                                    HistoryEntry {
                                        category: cat,
                                        from_value: input_str,
                                        from_unit: from_label,
                                        to_value: res.formatted,
                                        to_unit: to_label,
                                    },
                                );
                                if h.len() > 10 {
                                    h.pop();
                                }
                                history.set(h);
                            }
                        }
                        is_converting.set(false);
                    });
                }
                UnitCategory::Time => {
                    let from = (*time_from).clone();
                    let to = (*time_to).clone();
                    let from_label = from.label().to_string();
                    let to_label = to.label().to_string();
                    spawn_local(async move {
                        let args =
                            serde_wasm_bindgen::to_value(&ConvertTimeArgs { value, from, to })
                                .unwrap();
                        let result = invoke("convert_time_cmd", args).await;
                        if let Ok(res) = serde_wasm_bindgen::from_value::<ConversionResult>(result)
                        {
                            if res.success {
                                result_value.set(res.formatted.clone());
                                let mut h = (*history).clone();
                                h.insert(
                                    0,
                                    HistoryEntry {
                                        category: cat,
                                        from_value: input_str,
                                        from_unit: from_label,
                                        to_value: res.formatted,
                                        to_unit: to_label,
                                    },
                                );
                                if h.len() > 10 {
                                    h.pop();
                                }
                                history.set(h);
                            }
                        }
                        is_converting.set(false);
                    });
                }
                UnitCategory::Area => {
                    let from = (*area_from).clone();
                    let to = (*area_to).clone();
                    let from_label = from.label().to_string();
                    let to_label = to.label().to_string();
                    spawn_local(async move {
                        let args =
                            serde_wasm_bindgen::to_value(&ConvertAreaArgs { value, from, to })
                                .unwrap();
                        let result = invoke("convert_area_cmd", args).await;
                        if let Ok(res) = serde_wasm_bindgen::from_value::<ConversionResult>(result)
                        {
                            if res.success {
                                result_value.set(res.formatted.clone());
                                let mut h = (*history).clone();
                                h.insert(
                                    0,
                                    HistoryEntry {
                                        category: cat,
                                        from_value: input_str,
                                        from_unit: from_label,
                                        to_value: res.formatted,
                                        to_unit: to_label,
                                    },
                                );
                                if h.len() > 10 {
                                    h.pop();
                                }
                                history.set(h);
                            }
                        }
                        is_converting.set(false);
                    });
                }
                UnitCategory::Volume => {
                    let from = (*volume_from).clone();
                    let to = (*volume_to).clone();
                    let from_label = from.label().to_string();
                    let to_label = to.label().to_string();
                    spawn_local(async move {
                        let args =
                            serde_wasm_bindgen::to_value(&ConvertVolumeArgs { value, from, to })
                                .unwrap();
                        let result = invoke("convert_volume_cmd", args).await;
                        if let Ok(res) = serde_wasm_bindgen::from_value::<ConversionResult>(result)
                        {
                            if res.success {
                                result_value.set(res.formatted.clone());
                                let mut h = (*history).clone();
                                h.insert(
                                    0,
                                    HistoryEntry {
                                        category: cat,
                                        from_value: input_str,
                                        from_unit: from_label,
                                        to_value: res.formatted,
                                        to_unit: to_label,
                                    },
                                );
                                if h.len() > 10 {
                                    h.pop();
                                }
                                history.set(h);
                            }
                        }
                        is_converting.set(false);
                    });
                }
            }
        })
    };

    let on_copy_result = {
        let result_value = result_value.clone();
        let copied = copied.clone();
        Callback::from(move |_| {
            let value = (*result_value).clone();
            if value.is_empty() {
                return;
            }
            let copied = copied.clone();
            if let Some(win) = window() {
                let clipboard = win.navigator().clipboard();
                spawn_local(async move {
                    let _ =
                        wasm_bindgen_futures::JsFuture::from(clipboard.write_text(&value)).await;
                    copied.set(true);
                    let copied_reset = copied.clone();
                    gloo_timers::callback::Timeout::new(2000, move || {
                        copied_reset.set(false);
                    })
                    .forget();
                });
            }
        })
    };

    let on_clear_history = {
        let history = history.clone();
        Callback::from(move |_| {
            history.set(Vec::new());
        })
    };

    let render_unit_selectors = || -> Html {
        match *category {
            UnitCategory::Length => {
                let on_from_change = {
                    let length_from = length_from.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Centimeter" => LengthUnit::Centimeter,
                            "Millimeter" => LengthUnit::Millimeter,
                            "Kilometer" => LengthUnit::Kilometer,
                            "Inch" => LengthUnit::Inch,
                            "Feet" => LengthUnit::Feet,
                            "Yard" => LengthUnit::Yard,
                            "Mile" => LengthUnit::Mile,
                            _ => LengthUnit::Meter,
                        };
                        length_from.set(unit);
                    })
                };
                let on_to_change = {
                    let length_to = length_to.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Centimeter" => LengthUnit::Centimeter,
                            "Millimeter" => LengthUnit::Millimeter,
                            "Kilometer" => LengthUnit::Kilometer,
                            "Inch" => LengthUnit::Inch,
                            "Feet" => LengthUnit::Feet,
                            "Yard" => LengthUnit::Yard,
                            "Mile" => LengthUnit::Mile,
                            _ => LengthUnit::Meter,
                        };
                        length_to.set(unit);
                    })
                };
                html! {
                    <>
                        <select class="form-select" onchange={on_from_change}>
                            { for LengthUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*length_from == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                        <select class="form-select" onchange={on_to_change}>
                            { for LengthUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*length_to == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                    </>
                }
            }
            UnitCategory::Weight => {
                let on_from_change = {
                    let weight_from = weight_from.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Gram" => WeightUnit::Gram,
                            "Milligram" => WeightUnit::Milligram,
                            "Pound" => WeightUnit::Pound,
                            "Ounce" => WeightUnit::Ounce,
                            "Ton" => WeightUnit::Ton,
                            _ => WeightUnit::Kilogram,
                        };
                        weight_from.set(unit);
                    })
                };
                let on_to_change = {
                    let weight_to = weight_to.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Gram" => WeightUnit::Gram,
                            "Milligram" => WeightUnit::Milligram,
                            "Pound" => WeightUnit::Pound,
                            "Ounce" => WeightUnit::Ounce,
                            "Ton" => WeightUnit::Ton,
                            _ => WeightUnit::Kilogram,
                        };
                        weight_to.set(unit);
                    })
                };
                html! {
                    <>
                        <select class="form-select" onchange={on_from_change}>
                            { for WeightUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*weight_from == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                        <select class="form-select" onchange={on_to_change}>
                            { for WeightUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*weight_to == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                    </>
                }
            }
            UnitCategory::DataSize => {
                let on_from_change = {
                    let data_from = data_from.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Kilobyte" => DataSizeUnit::Kilobyte,
                            "Megabyte" => DataSizeUnit::Megabyte,
                            "Gigabyte" => DataSizeUnit::Gigabyte,
                            "Terabyte" => DataSizeUnit::Terabyte,
                            "Petabyte" => DataSizeUnit::Petabyte,
                            "Kibibyte" => DataSizeUnit::Kibibyte,
                            "Mebibyte" => DataSizeUnit::Mebibyte,
                            "Gibibyte" => DataSizeUnit::Gibibyte,
                            "Tebibyte" => DataSizeUnit::Tebibyte,
                            _ => DataSizeUnit::Byte,
                        };
                        data_from.set(unit);
                    })
                };
                let on_to_change = {
                    let data_to = data_to.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Kilobyte" => DataSizeUnit::Kilobyte,
                            "Megabyte" => DataSizeUnit::Megabyte,
                            "Gigabyte" => DataSizeUnit::Gigabyte,
                            "Terabyte" => DataSizeUnit::Terabyte,
                            "Petabyte" => DataSizeUnit::Petabyte,
                            "Kibibyte" => DataSizeUnit::Kibibyte,
                            "Mebibyte" => DataSizeUnit::Mebibyte,
                            "Gibibyte" => DataSizeUnit::Gibibyte,
                            "Tebibyte" => DataSizeUnit::Tebibyte,
                            _ => DataSizeUnit::Byte,
                        };
                        data_to.set(unit);
                    })
                };
                html! {
                    <>
                        <select class="form-select" onchange={on_from_change}>
                            { for DataSizeUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*data_from == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                        <select class="form-select" onchange={on_to_change}>
                            { for DataSizeUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*data_to == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                    </>
                }
            }
            UnitCategory::Temperature => {
                let on_from_change = {
                    let temp_from = temp_from.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Fahrenheit" => TemperatureUnit::Fahrenheit,
                            "Kelvin" => TemperatureUnit::Kelvin,
                            _ => TemperatureUnit::Celsius,
                        };
                        temp_from.set(unit);
                    })
                };
                let on_to_change = {
                    let temp_to = temp_to.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Fahrenheit" => TemperatureUnit::Fahrenheit,
                            "Kelvin" => TemperatureUnit::Kelvin,
                            _ => TemperatureUnit::Celsius,
                        };
                        temp_to.set(unit);
                    })
                };
                html! {
                    <>
                        <select class="form-select" onchange={on_from_change}>
                            { for TemperatureUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*temp_from == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                        <select class="form-select" onchange={on_to_change}>
                            { for TemperatureUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*temp_to == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                    </>
                }
            }
            UnitCategory::Time => {
                let on_from_change = {
                    let time_from = time_from.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Minute" => TimeUnit::Minute,
                            "Hour" => TimeUnit::Hour,
                            "Day" => TimeUnit::Day,
                            "Week" => TimeUnit::Week,
                            "Month" => TimeUnit::Month,
                            "Year" => TimeUnit::Year,
                            _ => TimeUnit::Second,
                        };
                        time_from.set(unit);
                    })
                };
                let on_to_change = {
                    let time_to = time_to.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Minute" => TimeUnit::Minute,
                            "Hour" => TimeUnit::Hour,
                            "Day" => TimeUnit::Day,
                            "Week" => TimeUnit::Week,
                            "Month" => TimeUnit::Month,
                            "Year" => TimeUnit::Year,
                            _ => TimeUnit::Second,
                        };
                        time_to.set(unit);
                    })
                };
                html! {
                    <>
                        <select class="form-select" onchange={on_from_change}>
                            { for TimeUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*time_from == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                        <select class="form-select" onchange={on_to_change}>
                            { for TimeUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*time_to == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                    </>
                }
            }
            UnitCategory::Area => {
                let on_from_change = {
                    let area_from = area_from.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "SquareKilometer" => AreaUnit::SquareKilometer,
                            "SquareCentimeter" => AreaUnit::SquareCentimeter,
                            "SquareFeet" => AreaUnit::SquareFeet,
                            "SquareInch" => AreaUnit::SquareInch,
                            "Hectare" => AreaUnit::Hectare,
                            "Acre" => AreaUnit::Acre,
                            "Tsubo" => AreaUnit::Tsubo,
                            _ => AreaUnit::SquareMeter,
                        };
                        area_from.set(unit);
                    })
                };
                let on_to_change = {
                    let area_to = area_to.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "SquareKilometer" => AreaUnit::SquareKilometer,
                            "SquareCentimeter" => AreaUnit::SquareCentimeter,
                            "SquareFeet" => AreaUnit::SquareFeet,
                            "SquareInch" => AreaUnit::SquareInch,
                            "Hectare" => AreaUnit::Hectare,
                            "Acre" => AreaUnit::Acre,
                            "Tsubo" => AreaUnit::Tsubo,
                            _ => AreaUnit::SquareMeter,
                        };
                        area_to.set(unit);
                    })
                };
                html! {
                    <>
                        <select class="form-select" onchange={on_from_change}>
                            { for AreaUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*area_from == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                        <select class="form-select" onchange={on_to_change}>
                            { for AreaUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*area_to == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                    </>
                }
            }
            UnitCategory::Volume => {
                let on_from_change = {
                    let volume_from = volume_from.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Milliliter" => VolumeUnit::Milliliter,
                            "CubicMeter" => VolumeUnit::CubicMeter,
                            "CubicCentimeter" => VolumeUnit::CubicCentimeter,
                            "Gallon" => VolumeUnit::Gallon,
                            "Quart" => VolumeUnit::Quart,
                            "Pint" => VolumeUnit::Pint,
                            "Cup" => VolumeUnit::Cup,
                            _ => VolumeUnit::Liter,
                        };
                        volume_from.set(unit);
                    })
                };
                let on_to_change = {
                    let volume_to = volume_to.clone();
                    let result_value = result_value.clone();
                    Callback::from(move |e: Event| {
                        result_value.set(String::new());
                        let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                        let unit = match select.value().as_str() {
                            "Milliliter" => VolumeUnit::Milliliter,
                            "CubicMeter" => VolumeUnit::CubicMeter,
                            "CubicCentimeter" => VolumeUnit::CubicCentimeter,
                            "Gallon" => VolumeUnit::Gallon,
                            "Quart" => VolumeUnit::Quart,
                            "Pint" => VolumeUnit::Pint,
                            "Cup" => VolumeUnit::Cup,
                            _ => VolumeUnit::Liter,
                        };
                        volume_to.set(unit);
                    })
                };
                html! {
                    <>
                        <select class="form-select" onchange={on_from_change}>
                            { for VolumeUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*volume_from == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                        <select class="form-select" onchange={on_to_change}>
                            { for VolumeUnit::all().iter().map(|u| {
                                let value = format!("{:?}", u);
                                html! {
                                    <option value={value.clone()} selected={*volume_to == *u}>
                                        {u.label()}
                                    </option>
                                }
                            })}
                        </select>
                    </>
                }
            }
        }
    };

    html! {
        <div class="unit-converter">
            <div class="section unit-category-section">
                <h3>{i18n.t("unit_converter.category_select")}</h3>
                <div class="category-grid">
                    { for UnitCategory::all().iter().map(|cat| {
                        let on_click = {
                            let on_category_change = on_category_change.clone();
                            let cat = *cat;
                            Callback::from(move |_| on_category_change.emit(cat))
                        };
                        let label = i18n.t(cat.translation_key());
                        html! {
                            <button
                                class={classes!("category-btn", (*category == *cat).then_some("active"))}
                                onclick={on_click}
                            >
                                <span class="category-icon">{cat.icon()}</span>
                                <span class="category-label">{label}</span>
                            </button>
                        }
                    })}
                </div>
            </div>

            <div class="section unit-convert-section">
                <h3>{i18n.t("unit_converter.convert_section")}</h3>
                <div class="convert-form">
                    <div class="convert-input-group">
                        <input
                            type="number"
                            class="form-input convert-input"
                            placeholder={i18n.t("unit_converter.input_placeholder")}
                            value={(*input_value).clone()}
                            oninput={on_input_change}
                            step="any"
                        />
                        {render_unit_selectors()}
                    </div>
                    <div class="convert-actions">
                        <button
                            class="swap-btn"
                            onclick={on_swap_units}
                            title={i18n.t("unit_converter.swap_tooltip")}
                        >
                            {"⇄"}
                        </button>
                        <button
                            class="primary-btn convert-btn"
                            onclick={on_convert}
                            disabled={*is_converting || input_value.is_empty()}
                        >
                            if *is_converting {
                                <span class="processing">
                                    <span class="spinner"></span>
                                    {i18n.t("common.converting")}
                                </span>
                            } else {
                                {i18n.t("common.convert")}
                            }
                        </button>
                    </div>
                </div>
            </div>

            if !result_value.is_empty() {
                <div class="section unit-result-section">
                    <h3>{i18n.t("unit_converter.result_section")}</h3>
                    <div class="result-display">
                        <code class="result-value">{&*result_value}</code>
                        <button
                            class={classes!("copy-btn", (*copied).then_some("copied"))}
                            onclick={on_copy_result}
                        >
                            if *copied {
                                {"✓"}
                            } else {
                                {"📋"}
                            }
                        </button>
                    </div>
                </div>
            }

            if !history.is_empty() {
                <div class="section unit-history-section">
                    <div class="history-header">
                        <h3>{i18n.t("unit_converter.history_section")}</h3>
                        <button class="toolbar-btn" onclick={on_clear_history}>
                            {i18n.t("unit_converter.history_clear")}
                        </button>
                    </div>
                    <div class="history-list">
                        { for (*history).iter().map(|entry| {
                            html! {
                                <div class="history-item">
                                    <span class="history-icon">{entry.category.icon()}</span>
                                    <span class="history-from">
                                        {&entry.from_value}{" "}{&entry.from_unit}
                                    </span>
                                    <span class="history-arrow">{"→"}</span>
                                    <span class="history-to">
                                        {&entry.to_value}{" "}{&entry.to_unit}
                                    </span>
                                </div>
                            }
                        })}
                    </div>
                </div>
            }
        </div>
    }
}
