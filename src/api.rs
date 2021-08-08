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
pub struct Hour {
    /// Time of data calculation, unix, UTC
    pub dt: i64,
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
    /// Cloudiness, %
    pub clouds: i64,
    /// Current UV index
    pub uvi: f64,
    /// Visibility, meter
    pub visibility: i64,
    /// Wind speed. Unit Default: meter/sec, Metric: meter/sec, Imperial: miles/hour.
    pub wind_speed: f64,
    /// Wind direction, degrees (meteorological)
    pub wind_deg: f64, 
    /// Wind gust. Unit Default: metre/sec, metric: metre/sec, imperial: miles/hour.
    pub wind_gust: Option<f64>,    
    /// vector with one item of weather condition descriptions
    pub weather: Vec<Weather>,
    // Probability of precipitation    
    pub pop: f64, 
    // Rain volume for last hour, mm
    pub rain: Option<Volume>,
    // Snow volume for last hour, mm
    pub snow: Option<Volume>,
}

#[derive(Deserialize, Debug)]
pub struct DailyTemp {
    /// Morning temperature.
    pub morn: f64,
    /// Day temperature.
    pub day: f64,  
    /// Evening temperature.
    pub eve: f64,  
    /// Night temperature.
    pub night: f64,  
    /// Min daily temperature.
    pub min: f64,
    /// Max daily temperature.
    pub max: f64,  
}

#[derive(Deserialize, Debug)]
pub struct DailyFeelsLike {
    /// Morning temperature.
    pub morn: f64,
    /// Day temperature.
    pub day: f64,  
    /// Evening temperature.
    pub eve: f64,  
    /// Night temperature.
    pub night: f64,  
}

#[derive(Deserialize, Debug)]
/// current weather report in a nested struct
pub struct Day {
    /// Time of data calculation, unix, UTC
    pub dt: i64,
    /// Sunrise time, unix, UTC
    pub sunrise: i64,
    /// Sunset time, unix, UTC
    pub sunset: i64,
    /// The time of when the moon rises for this day, Unix, UTC
    pub moonrise: i64,
    /// The time of when the moon sets for this day, Unix, UTC
    pub moonset: i64,
    ///  Moon phase. 0 and 1 are 'new moon', 0.25 is 'first quarter moon', 
    ///  0.5 is 'full moon' and 0.75 is 'last quarter moon'.
    pub moon_phase: f64,    
    /// Temperature. Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub temp: DailyTemp,
    /// Temperature. This temperature parameter accounts for the human perception of weather.
    /// Unit Default: Kelvin, Metric: Celsius, Imperial: Fahrenheit.
    pub feels_like: DailyFeelsLike,
    /// Atmospheric pressure (on the sea level, if there is no sea_level or grnd_level data), hPa
    pub pressure: f64,
    /// Humidity, %
    pub humidity: f64,
    /// Dew Point (see Temperature)
    pub dew_point: f64,
    /// Cloudiness, %
    pub clouds: i64,
    /// Current UV index
    pub uvi: f64,
    /// Wind speed. Unit Default: meter/sec, Metric: meter/sec, Imperial: miles/hour.
    pub wind_speed: f64,
    /// Wind direction, degrees (meteorological)
    pub wind_deg: f64, 
    /// Wind gust. Unit Default: metre/sec, metric: metre/sec, imperial: miles/hour.
    pub wind_gust: Option<f64>,    
    /// vector with one item of weather condition descriptions
    pub weather: Vec<Weather>,
    // Probability of precipitation    
    pub pop: f64, 
    // Precipitation volume, mm
    pub rain: Option<f64>,
    // Snow volume, mm
    pub snow: Option<f64>,
}

#[derive(Deserialize, Debug)]
/// current weather report in a nested struct
pub struct Minute {
    /// Time of data calculation, unix, UTC
    pub dt: i64,
    //Precipitation volume, mm
    pub precipitation: Option<f64>
}

#[derive(Deserialize, Debug)]
/// current weather report in a nested struct
pub struct Alert {
    /// Name of the alert source.
    pub sender_name: String,
    /// Alert event name.
    pub event: String,
    /// Date and time of the start of the alert, Unix, UTC
    pub start: i64,
    /// Date and time of the end of the alert, Unix, UTC
    pub end: i64,
    /// Description of the alert.
    pub description: String,        
    /// Type of severe weather
    pub tags: String
}

#[derive(Deserialize, Debug)]
/// current weather report in a nested struct
pub struct Current {
    /// Time of data calculation, unix, UTC
    pub dt: i64,
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
    /// Cloudiness, %
    pub clouds: i64,
    /// Current UV index
    pub uvi: f64,
    /// Visibility, meter
    pub visibility: i64,
    /// Wind speed. Unit Default: meter/sec, Metric: meter/sec, Imperial: miles/hour.
    pub wind_speed: f64,
    /// Wind direction, degrees (meteorological)
    pub wind_deg: f64, 
    /// Wind gust. Unit Default: metre/sec, metric: metre/sec, imperial: miles/hour.
    pub wind_gust: Option<f64>,    
    /// vector with one item of weather condition descriptions
    pub weather: Vec<Weather>,
    //Rain volume for last hour, mm
    pub rain: Option<Volume>,
    //Snow volume for last hour, mm
    pub snow: Option<Volume>,
}

#[derive(Deserialize, Debug)]
/// current weather report in a nested struct
pub struct OneCall {
    /// geo location, latitude
    pub lat: f64,
    /// geo location, longitude
    pub lon: f64,
    /// Shift in seconds from UTC
    pub timezone: String,
    /// Shift in seconds from UTC
    pub timezone_offset: i64,
    /// Current Weather
    pub current: Option<Current>,
    /// Hourly Weather
    pub hourly: Option<Vec<Hour>>,
    /// Minute Weather
    pub minutely: Option<Vec<Minute>>,
    /// Daily forecast weather
    pub daily: Option<Vec<Day>>,
    /// National weather alerts data from major national weather warning systems
    pub alerts: Option<Vec<Alert>>
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
