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

#[derive(Deserialize, Debug)]
pub struct Coord {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    pub id: u64,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct Main {
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: f64,
    pub humidity: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub sea_level: Option<f64>,
    pub grnd_level: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f64,
    pub deg: f64,
    pub gust: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    pub all: f64,
}

#[derive(Deserialize, Debug)]
pub struct Volume {
    #[serde(rename = "1h")]
    pub h1: Option<f64>,
    #[serde(rename = "3h")]
    pub h3: Option<f64>,
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    #[serde(rename = "type")]
    pub type_: Option<u64>,
    pub id: Option<u64>,
    pub message: Option<f64>,
    pub country: String,
    pub sunrise: u64,
    pub sunset: u64,
}

#[derive(Deserialize, Debug)]
pub struct CurrentWeather {
    pub coord: Coord,
    pub weather: Vec<Weather>,
    pub base: String,
    pub main: Main,
    pub visibility: u64,
    pub wind: Wind,
    pub clouds: Clouds,
    pub rain: Option<Volume>,
    pub snow: Option<Volume>,
    pub dt: i64,
    pub sys: Sys,
    pub timezone: i64,
    pub id: u64,
    pub name: String,
    pub cod: u64,
}

type Receiver = mpsc::Receiver<Result<CurrentWeather, String>>;

pub const LOADING: &str = "loading...";

// start weather fetching which will spawn a thread that signals updates from OWM in json format
// via the returned receiver
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

// get some weather update or None (if there is nothing new)
pub fn update(receiver: &Receiver) -> Option<Result<CurrentWeather, String>> {
    match receiver.try_recv() {
        Ok(response) => Some(response),
        Err(_e) => None,
    }
}

// get weather just once
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

pub mod blocking {
    use super::*;
    // blocking variant of weather()
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
