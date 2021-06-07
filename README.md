# openweathermap [![Rust](https://github.com/fightling/openweathermap/actions/workflows/rust.yml/badge.svg)](https://github.com/fightling/openweathermap/actions/workflows/rust.yml)

...is a *rust crate* which lets you easily access current weather data from [OpenWeatherMap](https://openweathermap.org/). This is an *unofficial* extension I have made to learn *rust* a little but I hope you have fun with it.

## Contents

<!-- MDTOC maxdepth:6 firsth1:2 numbering:0 flatten:0 bullets:1 updateOnSave:1 -->

- [Contents](#contents)   
- [How to use](#how-to-use)   
   - [Get continuous weather updates](#get-continuous-weather-updates)   
      - [First: Start polling](#first-start-polling)   
      - [Then: Get weather updates](#then-get-weather-updates)   
         - [Nothing New: `None`](#nothing-new-none)   
         - [Weather Update: `CurrentWeather`](#weather-update-currentweather)   
         - [Some Error: `Err`](#some-error-err)   
   - [Get weather just once](#get-weather-just-once)   
- [Reference Documentation](#reference-documentation)   
- [Links](#links)   
   - [Website](#website)   
   - [*github* repository](#github-repository)   
   - [on *crates.io*](#on-cratesio)   
- [License](#license)   

<!-- /MDTOC -->

## How to use

First add this crate to your dependencies in you `Cargo.toml` file:

```toml
[dependencies]
openweathermap = "0.2.4"
```
### Get continuous weather updates

Then use the crate in your rust source file by calling `openweathermap::init()` which returns a receiver object.
You can then use this receiver object to call `openweathermap::update()` to get weather updates like in the following example:

```rust
extern crate openweathermap;

use openweathermap::{init,update};

fn main() {
    // start our observatory via OWM
    let receiver = &init("Berlin,DE", "metric", "en", "<APIKEY>", 10);
    loop {
        match update(receiver) {
            Some(response) => match response {
                Ok(current) => println!(
                    "Today's weather in {} is {}",
                    current.name.as_str(),
                    current.weather[0].main.as_str()
                ),
                Err(e) => println!("Could not fetch weather because: {}", e),
            },
            None => (),
        }
    }
}
```

#### First: Start polling

`init()` spawns a thread which then will periodically poll *OpenWeatherMap* for the latest current weather report.
You then can use `update()` to ask for it.

#### Then: Get weather updates

There are three possible kinds of result you get from `update()` which you will have to face:

##### Nothing New: `None`

`update()` returns `None` if there is currently no new update available.
Which means: **You wont get any update twice!**
In other words: `update()` is not caching the last weather update for you.

##### Weather Update: `CurrentWeather`

If a new update was downloaded by the polling thread `update()` returns some `CurrentWeather` object.
`CurrentWeather` is a nested `struct` with the already parsed json properties.
Those are well described [here](https://openweathermap.org/current#parameter).

##### Some Error: `Err`
On error `update()` returns some `String` object which includes a brief error description.

Errors may occur...
- initially while **there is no update yet** you will get an `Err` which includes exactly the String `"loading..."` (predefined in `openweathermap::LOADING`).
- if a **server error** response was received (e.g. `401 Unauthorized` if an **invalid API key** was used).
- on **json errors** while parsing the response from *OpenWeatherMap*.

### Get weather just once

If you need the weather just once you may use the method `weather()` which envelopes `init()` and `update()` into one single synchronous or asynchronous call.
After the first successful weather update the spawned thread will stop immediately and you get the result in return.

```rust
extern crate openweathermap;
use openweathermap::blocking::weather;

fn main() {
    // start our observatory via OWM
    match &weather("Berlin,DE", "metric", "en", "<APIKEY>") {
        Ok(current) => println!(
            "Today's weather in {} is {}",
            current.name.as_str(),
            current.weather[0].main.as_str()
        ),
        Err(e) => println!("Could not fetch weather because: {}", e),
    }
}

```

There is a *blocking* and a *non-blocking* variant of `weather()`:

- The above example uses the synchronous (*blocking*) variant `openweathermap::blocking::weather` which wont return until there is a new update.
- If you like to deal with the returned *future* by yourself just use `openweathermap::weather` and asynchronously await the result until there is any.

## Reference Documentation

Beside this introduction there is a reference documentation which can be found [here](https://docs.rs/openweathermap).

## Links

### Website

This README tastes better at [openweathermap.thats-software.com](https://openweathermap.thats-software.com).

### *github* repository

For the source code see [this repository](https://github.com/fightling/openweathermap) at *github.com*.

### on *crates.io*

Published at [*crates.io*](https://crates.io/crates/openweathermap).

## License

openweathermap is licensed under the *MIT license* (LICENSE-MIT or http://opensource.org/licenses/MIT)
