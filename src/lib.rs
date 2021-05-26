extern crate reqwest;
extern crate serde_json;

use futures::executor;
use http::StatusCode;
use regex::Regex;
use serde::Deserialize;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests;

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

/// Receiver object you get from `init()` and have top handle to `update()`.
pub type Receiver = mpsc::Receiver<Result<CurrentWeather, String>>;
/// Loading error messaage you get at the first call of `update()`.
pub const LOADING: &str = "loading...";

/// Spawns a thread which fetches the current weather from
/// [openweathermap.org](https://openweathermap.org) periodically.
/// #### Parameters
/// - `location`: Can be a city name, a city ID or a geographical coordinate:
///     - city name: may be followed by comma separated state code and/or country code (e.g. `"Berlin,DE"`).
///     - city ID: which can be found at [this](https://openweathermap.org/find) where you will get link that includes the ID
///         - e.g. `"2950159"` for Berlin, Germany
///     - coordinates: given by comma separated latitude and longitude (e.g. `"52.5244,13.4105"`). |
/// - `units`: One of the following:
///     - `"metric"`: meters, m/s, °C, etc.
///     - `"imperial"`: miles, mi/s, °F, etc.
///     - `"standard"`: meters, m/s, K, etc.
/// - `lang`: Language code:
///     - `"en"`: for English
///     - `"de"`: for German
///     - see [this list](https://openweathermap.org/current#multi) for all available language codes
/// - `api_key`: Your API key which you can get [here](https://openweathermap.org/price)
/// - `poll_mins`: Update interval:
///     - `> 0`: duration of poll period in minutes (`10` is recommended)
///     - `= 0`: thread will terminate after the first successful update.
/// #### Return value
/// - `openweathermap::Receiver`: Handle this to `openweathermap::update()` to get the latest weather update.
///
///    The return value is a `mpsc` *channel receiver*:
///    ```rust
///     pub type Receiver = std::sync::mpsc::Receiver<Result<openweathermap::CurrentWeather, String>>;
///    ```

pub fn init(location: &str, units: &str, lang: &str, api_key: &str, poll_mins: u64) -> Receiver {
    // generate correct request URL depending on city is id or name
    let url = match location.parse::<u64>().is_ok() {
        true => format!(
            "https://api.openweathermap.org/data/2.5/weather?id={}&units={}&lang={}&appid={}",
            location, units, lang, api_key
        ),
        false => {
            let re = Regex::new(r"(-?\d+\.\d+)\s*,\s*(-?\d+\.\d+)").unwrap();
            match re.captures(&location) {
                Some(caps) => format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units={}&lang={}&appid={}",
                            caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str(), units, lang, api_key ),
                None => format!(
                            "https://api.openweathermap.org/data/2.5/weather?q={}&units={}&lang={}&appid={}",
                            location, units, lang, api_key ),
            }
        }
    };
    // fork thread that continuously fetches weather updates every <poll_mins> minutes
    let period = Duration::from_secs(60 * poll_mins);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(Err(LOADING.to_string())).unwrap_or(());
        loop {
            match reqwest::blocking::get(&url) {
                Ok(response) => match response.status() {
                    StatusCode::OK => match serde_json::from_str(&response.text().unwrap()) {
                        Ok(w) => {
                            tx.send(Ok(w)).unwrap_or(());
                            if period == Duration::new(0, 0) {
                                break;
                            }
                            thread::sleep(period);
                        }
                        Err(e) => tx.send(Err(e.to_string())).unwrap_or(()),
                    },
                    _ => tx.send(Err(response.status().to_string())).unwrap_or(()),
                },
                Err(_e) => (),
            }
        }
    });
    // return receiver that provides the updated weather as json string
    return rx;
}

/// Get current weather update that the spawned thread could fetched.
/// #### Parameters
/// - `receiver`: the *channel receiver* from preceded call to `openweathermap::init()`
/// #### Returng value
/// - ⇒ `None`: No update available
/// - ⇒ `Some(Result)`: Update available
///     - ⇒ `Ok(CurrentWeather)`: Weather information in a nested struct called `CurrentWeather`
///         (see also [*OpenWeatherMap* documentation](https://openweathermap.org/current#parameter) for details)
///     - ⇒ `Err(String)`: Error message about any occured http or json issue
///         - e.g. `401 Unauthorized`: if your API key is invalid
///         - some json parser error message if response from OpenWeatherMap could not be parsed
pub fn update(receiver: &Receiver) -> Option<Result<CurrentWeather, String>> {
    match receiver.try_recv() {
        Ok(response) => Some(response),
        Err(_e) => None,
    }
}

/// Fetch current weather update once and stop thread immediately after success.
/// Returns the result in a *future*.
/// #### Parameters
/// - `location`: Can be a city name, a city ID or a geographical coordinate:
///     - city name: may be followed by comma separated state code and/or country code (e.g. `"Berlin,DE"`).
///     - city ID: which can be found at [this](https://openweathermap.org/find) where you will get link that includes the ID
///         - e.g. `"2950159"` for Berlin, Germany
///     - coordinates: given by comma separated latitude and longitude (e.g. `"52.5244,13.4105"`). |
/// - `units`: One of the following:
///     - `"metric"`: meters, m/s, °C, etc.
///     - `"imperial"`: miles, mi/s, °F, etc.
///     - `"standard"`: meters, m/s, K, etc.
/// - `lang`: Language code:
///     - `"en"`: for English
///     - `"de"`: for German
///     - see [this list](https://openweathermap.org/current#multi) for all available language codes
/// - `api_key`: Your API key which you can get [here](https://openweathermap.org/price)
/// #### Return value
/// - ⇒ `Ok(CurrentWeather)`: weather information in a nested struct called `CurrentWeather`
///     (see also [*OpenWeatherMap* documentation](https://openweathermap.org/current#parameter) for details)
/// - ⇒ `Err(String)`: Error message about any occured http or json issue
///         - e.g. `401 Unauthorized` if your API key is invalid
///         - some json parser error message if response from OpenWeatherMap could not be parsed
pub async fn weather(
    location: &str,
    units: &str,
    lang: &str,
    api_key: &str,
) -> Result<CurrentWeather, String> {
    let r = init(location, units, lang, api_key, 0);
    loop {
        match update(&r) {
            Some(response) => match response {
                Ok(current) => return Ok(current),
                Err(e) => {
                    if e != LOADING {
                        return Err(e);
                    }
                }
            },
            None => (),
        }
    }
}

/// synchronous functions
pub mod blocking {
    use super::*;
    /// Fetches a weather update once and stops the thread immediately after success then returns the update.
    /// #### Parameters
    /// - `location`: Can be a city name, a city ID or a geographical coordinate:
    ///     - city name may be followed by comma separated state code and/or country code (e.g. `"Berlin,DE"`).
    ///     - city ID which can be found at [this](https://openweathermap.org/find) where you will get link that includes the ID
    ///         - e.g. `"2950159"` for Berlin, Germany
    ///     - coordinates given by comma separated latitude and longitude (e.g. `"52.5244,13.4105"`). |
    /// - `units`: One of the following:
    ///     - `"metric"`: meters, m/s, °C, etc.
    ///     - `"imperial"`: miles, mi/s, °F, etc.
    ///     - `"standard"`: meters, m/s, K, etc.
    /// - `lang`: Language code:
    ///     - `"en"`: for English
    ///     - `"de"`: for German
    ///     - see [this list](https://openweathermap.org/current#multi) for all available language codes
    /// - `api_key`: Your API key which you can get [here](https://openweathermap.org/price)
    /// #### Return value
    /// - ⇒ `Ok(CurrentWeather)`: weather information in a nested struct called `CurrentWeather`
    ///     (see also [*OpenWeatherMap* documentation](https://openweathermap.org/current#parameter) for details)
    /// - ⇒ `Err(String)`: Error message about any occured http or json issue
    ///         - e.g. `401 Unauthorized` if your API key is invalid
    ///         - some json parser error message if response from OpenWeatherMap could not be parsed
    pub fn weather(
        location: &str,
        units: &str,
        lang: &str,
        api_key: &str,
    ) -> Result<CurrentWeather, String> {
        // wait for result
        executor::block_on(super::weather(location, units, lang, api_key))
    }
}
