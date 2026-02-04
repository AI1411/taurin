use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnitCategory {
    Length,
    Weight,
    DataSize,
    Temperature,
    Time,
    Area,
    Volume,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WeightUnit {
    Kilogram,
    Gram,
    Milligram,
    Pound,
    Ounce,
    Ton,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionResult {
    pub success: bool,
    pub result: f64,
    pub formatted: String,
    pub error: Option<String>,
}

// Length conversion (base unit: meter)
fn length_to_meter(value: f64, unit: &LengthUnit) -> f64 {
    match unit {
        LengthUnit::Meter => value,
        LengthUnit::Centimeter => value / 100.0,
        LengthUnit::Millimeter => value / 1000.0,
        LengthUnit::Kilometer => value * 1000.0,
        LengthUnit::Inch => value * 0.0254,
        LengthUnit::Feet => value * 0.3048,
        LengthUnit::Yard => value * 0.9144,
        LengthUnit::Mile => value * 1609.344,
    }
}

fn meter_to_length(value: f64, unit: &LengthUnit) -> f64 {
    match unit {
        LengthUnit::Meter => value,
        LengthUnit::Centimeter => value * 100.0,
        LengthUnit::Millimeter => value * 1000.0,
        LengthUnit::Kilometer => value / 1000.0,
        LengthUnit::Inch => value / 0.0254,
        LengthUnit::Feet => value / 0.3048,
        LengthUnit::Yard => value / 0.9144,
        LengthUnit::Mile => value / 1609.344,
    }
}

pub fn convert_length(value: f64, from: LengthUnit, to: LengthUnit) -> ConversionResult {
    let meters = length_to_meter(value, &from);
    let result = meter_to_length(meters, &to);
    ConversionResult {
        success: true,
        result,
        formatted: format_number(result),
        error: None,
    }
}

// Weight conversion (base unit: kilogram)
fn weight_to_kg(value: f64, unit: &WeightUnit) -> f64 {
    match unit {
        WeightUnit::Kilogram => value,
        WeightUnit::Gram => value / 1000.0,
        WeightUnit::Milligram => value / 1_000_000.0,
        WeightUnit::Pound => value * 0.45359237,
        WeightUnit::Ounce => value * 0.028349523125,
        WeightUnit::Ton => value * 1000.0,
    }
}

fn kg_to_weight(value: f64, unit: &WeightUnit) -> f64 {
    match unit {
        WeightUnit::Kilogram => value,
        WeightUnit::Gram => value * 1000.0,
        WeightUnit::Milligram => value * 1_000_000.0,
        WeightUnit::Pound => value / 0.45359237,
        WeightUnit::Ounce => value / 0.028349523125,
        WeightUnit::Ton => value / 1000.0,
    }
}

pub fn convert_weight(value: f64, from: WeightUnit, to: WeightUnit) -> ConversionResult {
    let kg = weight_to_kg(value, &from);
    let result = kg_to_weight(kg, &to);
    ConversionResult {
        success: true,
        result,
        formatted: format_number(result),
        error: None,
    }
}

// Data size conversion (base unit: byte)
fn data_to_bytes(value: f64, unit: &DataSizeUnit) -> f64 {
    match unit {
        DataSizeUnit::Byte => value,
        DataSizeUnit::Kilobyte => value * 1000.0,
        DataSizeUnit::Megabyte => value * 1_000_000.0,
        DataSizeUnit::Gigabyte => value * 1_000_000_000.0,
        DataSizeUnit::Terabyte => value * 1_000_000_000_000.0,
        DataSizeUnit::Petabyte => value * 1_000_000_000_000_000.0,
        DataSizeUnit::Kibibyte => value * 1024.0,
        DataSizeUnit::Mebibyte => value * 1_048_576.0,
        DataSizeUnit::Gibibyte => value * 1_073_741_824.0,
        DataSizeUnit::Tebibyte => value * 1_099_511_627_776.0,
    }
}

fn bytes_to_data(value: f64, unit: &DataSizeUnit) -> f64 {
    match unit {
        DataSizeUnit::Byte => value,
        DataSizeUnit::Kilobyte => value / 1000.0,
        DataSizeUnit::Megabyte => value / 1_000_000.0,
        DataSizeUnit::Gigabyte => value / 1_000_000_000.0,
        DataSizeUnit::Terabyte => value / 1_000_000_000_000.0,
        DataSizeUnit::Petabyte => value / 1_000_000_000_000_000.0,
        DataSizeUnit::Kibibyte => value / 1024.0,
        DataSizeUnit::Mebibyte => value / 1_048_576.0,
        DataSizeUnit::Gibibyte => value / 1_073_741_824.0,
        DataSizeUnit::Tebibyte => value / 1_099_511_627_776.0,
    }
}

pub fn convert_data_size(value: f64, from: DataSizeUnit, to: DataSizeUnit) -> ConversionResult {
    let bytes = data_to_bytes(value, &from);
    let result = bytes_to_data(bytes, &to);
    ConversionResult {
        success: true,
        result,
        formatted: format_number(result),
        error: None,
    }
}

// Temperature conversion
pub fn convert_temperature(
    value: f64,
    from: TemperatureUnit,
    to: TemperatureUnit,
) -> ConversionResult {
    let celsius = match from {
        TemperatureUnit::Celsius => value,
        TemperatureUnit::Fahrenheit => (value - 32.0) * 5.0 / 9.0,
        TemperatureUnit::Kelvin => value - 273.15,
    };

    let result = match to {
        TemperatureUnit::Celsius => celsius,
        TemperatureUnit::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        TemperatureUnit::Kelvin => celsius + 273.15,
    };

    ConversionResult {
        success: true,
        result,
        formatted: format_number(result),
        error: None,
    }
}

// Time conversion (base unit: second)
fn time_to_seconds(value: f64, unit: &TimeUnit) -> f64 {
    match unit {
        TimeUnit::Second => value,
        TimeUnit::Minute => value * 60.0,
        TimeUnit::Hour => value * 3600.0,
        TimeUnit::Day => value * 86400.0,
        TimeUnit::Week => value * 604800.0,
        TimeUnit::Month => value * 2592000.0, // 30 days
        TimeUnit::Year => value * 31536000.0, // 365 days
    }
}

fn seconds_to_time(value: f64, unit: &TimeUnit) -> f64 {
    match unit {
        TimeUnit::Second => value,
        TimeUnit::Minute => value / 60.0,
        TimeUnit::Hour => value / 3600.0,
        TimeUnit::Day => value / 86400.0,
        TimeUnit::Week => value / 604800.0,
        TimeUnit::Month => value / 2592000.0,
        TimeUnit::Year => value / 31536000.0,
    }
}

pub fn convert_time(value: f64, from: TimeUnit, to: TimeUnit) -> ConversionResult {
    let seconds = time_to_seconds(value, &from);
    let result = seconds_to_time(seconds, &to);
    ConversionResult {
        success: true,
        result,
        formatted: format_number(result),
        error: None,
    }
}

// Area conversion (base unit: square meter)
fn area_to_sqm(value: f64, unit: &AreaUnit) -> f64 {
    match unit {
        AreaUnit::SquareMeter => value,
        AreaUnit::SquareKilometer => value * 1_000_000.0,
        AreaUnit::SquareCentimeter => value / 10_000.0,
        AreaUnit::SquareFeet => value * 0.09290304,
        AreaUnit::SquareInch => value * 0.00064516,
        AreaUnit::Hectare => value * 10_000.0,
        AreaUnit::Acre => value * 4046.8564224,
        AreaUnit::Tsubo => value * 3.305785,
    }
}

fn sqm_to_area(value: f64, unit: &AreaUnit) -> f64 {
    match unit {
        AreaUnit::SquareMeter => value,
        AreaUnit::SquareKilometer => value / 1_000_000.0,
        AreaUnit::SquareCentimeter => value * 10_000.0,
        AreaUnit::SquareFeet => value / 0.09290304,
        AreaUnit::SquareInch => value / 0.00064516,
        AreaUnit::Hectare => value / 10_000.0,
        AreaUnit::Acre => value / 4046.8564224,
        AreaUnit::Tsubo => value / 3.305785,
    }
}

pub fn convert_area(value: f64, from: AreaUnit, to: AreaUnit) -> ConversionResult {
    let sqm = area_to_sqm(value, &from);
    let result = sqm_to_area(sqm, &to);
    ConversionResult {
        success: true,
        result,
        formatted: format_number(result),
        error: None,
    }
}

// Volume conversion (base unit: liter)
fn volume_to_liter(value: f64, unit: &VolumeUnit) -> f64 {
    match unit {
        VolumeUnit::Liter => value,
        VolumeUnit::Milliliter => value / 1000.0,
        VolumeUnit::CubicMeter => value * 1000.0,
        VolumeUnit::CubicCentimeter => value / 1000.0,
        VolumeUnit::Gallon => value * 3.785411784,
        VolumeUnit::Quart => value * 0.946352946,
        VolumeUnit::Pint => value * 0.473176473,
        VolumeUnit::Cup => value * 0.2365882365,
    }
}

fn liter_to_volume(value: f64, unit: &VolumeUnit) -> f64 {
    match unit {
        VolumeUnit::Liter => value,
        VolumeUnit::Milliliter => value * 1000.0,
        VolumeUnit::CubicMeter => value / 1000.0,
        VolumeUnit::CubicCentimeter => value * 1000.0,
        VolumeUnit::Gallon => value / 3.785411784,
        VolumeUnit::Quart => value / 0.946352946,
        VolumeUnit::Pint => value / 0.473176473,
        VolumeUnit::Cup => value / 0.2365882365,
    }
}

pub fn convert_volume(value: f64, from: VolumeUnit, to: VolumeUnit) -> ConversionResult {
    let liters = volume_to_liter(value, &from);
    let result = liter_to_volume(liters, &to);
    ConversionResult {
        success: true,
        result,
        formatted: format_number(result),
        error: None,
    }
}

fn format_number(value: f64) -> String {
    if value.abs() < 0.000001 || value.abs() >= 1_000_000_000.0 {
        format!("{:.6e}", value)
    } else if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        let formatted = format!("{:.10}", value);
        formatted
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_conversion() {
        let result = convert_length(1.0, LengthUnit::Meter, LengthUnit::Centimeter);
        assert!((result.result - 100.0).abs() < 0.0001);

        let result = convert_length(1.0, LengthUnit::Feet, LengthUnit::Inch);
        assert!((result.result - 12.0).abs() < 0.0001);
    }

    #[test]
    fn test_weight_conversion() {
        let result = convert_weight(1.0, WeightUnit::Kilogram, WeightUnit::Gram);
        assert!((result.result - 1000.0).abs() < 0.0001);

        let result = convert_weight(1.0, WeightUnit::Pound, WeightUnit::Ounce);
        assert!((result.result - 16.0).abs() < 0.0001);
    }

    #[test]
    fn test_data_size_conversion() {
        let result = convert_data_size(1.0, DataSizeUnit::Gigabyte, DataSizeUnit::Megabyte);
        assert!((result.result - 1000.0).abs() < 0.0001);

        let result = convert_data_size(1.0, DataSizeUnit::Gibibyte, DataSizeUnit::Mebibyte);
        assert!((result.result - 1024.0).abs() < 0.0001);
    }

    #[test]
    fn test_temperature_conversion() {
        let result =
            convert_temperature(0.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit);
        assert!((result.result - 32.0).abs() < 0.0001);

        let result = convert_temperature(100.0, TemperatureUnit::Celsius, TemperatureUnit::Kelvin);
        assert!((result.result - 373.15).abs() < 0.0001);
    }

    #[test]
    fn test_time_conversion() {
        let result = convert_time(1.0, TimeUnit::Hour, TimeUnit::Minute);
        assert!((result.result - 60.0).abs() < 0.0001);

        let result = convert_time(1.0, TimeUnit::Day, TimeUnit::Hour);
        assert!((result.result - 24.0).abs() < 0.0001);
    }

    #[test]
    fn test_area_conversion() {
        let result = convert_area(1.0, AreaUnit::SquareKilometer, AreaUnit::Hectare);
        assert!((result.result - 100.0).abs() < 0.0001);
    }

    #[test]
    fn test_volume_conversion() {
        let result = convert_volume(1.0, VolumeUnit::Liter, VolumeUnit::Milliliter);
        assert!((result.result - 1000.0).abs() < 0.0001);
    }
}
