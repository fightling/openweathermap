extern crate reqwest;
extern crate serde_json;

use futures::executor;
use http::StatusCode;
use regex::Regex;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod api;
pub use api::*;

#[cfg(test)]
mod tests;

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
            "http://api.openweathermap.org/data/2.5/weather?id={}&units={}&lang={}&appid={}",
            location, units, lang, api_key
        ),
        false => {
            let re = Regex::new(r"(-?\d+\.\d+)\s*,\s*(-?\d+\.\d+)").unwrap();
            match re.captures(&location) {
                Some(caps) => format!("http://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&units={}&lang={}&appid={}",
                            caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str(), units, lang, api_key ),
                None => format!(
                            "http://api.openweathermap.org/data/2.5/weather?q={}&units={}&lang={}&appid={}",
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
