use serde::Deserialize;

/// Location coordinates
#[derive(Deserialize, Debug)]
pub struct Coord {
    /// geo location, longitude
    pub lon: f64,
    /// geo location, latitude
    pub lat: f64,
}

/// Weather condition description
#[derive(Deserialize, Debug)]
pub struct Weather {
    /// Weather condition id
    pub id: u64,
    /// Group of weather parameters (Rain, Snow, Extreme etc.)
    pub main: String,
    /// Weather condition
    pub description: String,
    /// Weather icon id
    pub icon: String,
}

/// Detailed weather report
#[derive(Deserialize, Debug)]
pub struct Main {
    /// Temperature. Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp: f64,
    /// Temperature. This temperature parameter accounts for the human perception of weather.
    /// Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub feels_like: f64,
    /// Atmospheric pressure (on the sea level, if there is no sea_level or grnd_level data), hPa
    pub pressure: f64,
    /// Humidity, %
    pub humidity: f64,
    /// Minimum temperature at the moment.
    /// This is minimal currently observed temperature (within large megalopolises and urban areas).
    /// Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp_min: f64,
    /// Maximum temperature at the moment.
    /// This is maximal currently observed temperature (within large megalopolises and urban areas).
    /// Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp_max: f64,
    /// Atmospheric pressure on the sea level, hPa
    pub sea_level: Option<f64>,
    /// Atmospheric pressure on the ground level, hPa
    pub grnd_level: Option<f64>,
}

/// Detailed wind report
#[derive(Deserialize, Debug)]
pub struct Wind {
    /// Wind speed. Unit Default: meter/sec, Metric: meter/sec, Imperial: miles/hour.
    pub speed: f64,
    /// Wind direction, degrees (meteorological)
    pub deg: f64,
    /// Wind gust. Unit Default: meter/sec, Metric: meter/sec, Imperial: miles/hour
    pub gust: Option<f64>,
}

/// Detailed clouds report
#[derive(Deserialize, Debug)]
pub struct Clouds {
    /// Cloudiness, %
    pub all: f64,
}

/// Rain or snow volume report
#[derive(Deserialize, Debug)]
pub struct Volume {
    /// Volume for the last 1 hour, mm
    #[serde(rename = "1h")]
    pub h1: Option<f64>,
    /// Volume for the last 3 hours, mm
    #[serde(rename = "3h")]
    pub h3: Option<f64>,
}

/// Additional information
#[derive(Deserialize, Debug)]
pub struct Sys {
    /// Internal parameter
    #[serde(rename = "type")]
    pub type_: Option<u64>,
    /// Internal parameter
    pub id: Option<u64>,
    /// Internal parameter
    pub message: Option<f64>,
    /// Country code (GB, JP etc.)
    pub country: String,
    /// Sunrise time, unix, UTC
    pub sunrise: i64,
    /// Sunset time, unix, UTC
    pub sunset: i64,
}

#[derive(Deserialize, Debug)]
/// current weather report in a nested struct
pub struct OneCallCurrent {
    /// Sunrise time, unix, UTC
    pub sunrise: i64,
    /// Sunset time, unix, UTC
    pub sunset: i64,
    /// Temperature. Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp: f64,
    /// Temperature. This temperature parameter accounts for the human perception of weather.
    /// Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub feels_like: f64,
    /// Atmospheric pressure (on the sea level, if there is no sea_level or grnd_level data), hPa
    pub pressure: f64,
    /// Humidity, %
    pub humidity: f64,
    /// Dew Point (see Temperature)
    pub dew_point: f64,
    /// UVI
    pub uvi: f64,
    /// Cloudiness, %
    pub clouds: f64,
    /// Visibility, meter
    pub visibility: u64,
    /// Wind speed. Unit Default: meter/sec, Metric: meter/sec, Imperial: miles/hour.
    pub wind_speed: f64,
    /// Wind direction, degrees (meteorological)
    pub wind_deg: f64,    
    /// vector with one item of weather condition descriptions
    pub weather: Vec<Weather>
}

#[derive(Deserialize, Debug)]
/// current weather report in a nested struct
pub struct OneCallCurrentWeather {
    /// geo location, latitude
    pub lat: f64,
    /// geo location, longitude
    pub lon: f64,
    /// Shift in seconds from UTC
    pub timezone: String,
    /// Shift in seconds from UTC
    pub timezone_offset: i64,
    /// Current Weather
    pub current: OneCallCurrent
}

#[derive(Deserialize, Debug)]
/// current weather report in a nested struct
pub struct CurrentWeather {
    /// report origin coordinates
    pub coord: Coord,
    /// vector with one item of weather condition descriptions
    pub weather: Vec<Weather>,
    /// Internal parameter
    pub base: String,
    /// detailed weather report
    pub main: Main,
    /// Visibility, meter
    pub visibility: u64,
    /// detailed wind report
    pub wind: Wind,
    /// detailed clouds report
    pub clouds: Clouds,
    /// detailed rain report
    pub rain: Option<Volume>,
    /// detailed snow report
    pub snow: Option<Volume>,
    /// Time of data calculation, unix, UTC
    pub dt: i64,
    /// additional information
    pub sys: Sys,
    /// Shift in seconds from UTC
    pub timezone: i64,
    /// City ID
    pub id: u64,
    /// City name
    pub name: String,
    /// Internal parameter
    pub cod: u64,
}
