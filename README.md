# openweathermap

This is a crate which lets you easily access current weather data from [OpenWeatherMap](https://openweathermap.org/).

## How to use

First add this crate to your dependencies in you `Cargo.toml` file:

```toml
[dependencies]
openweathermap = "0.0.2"
```

Then use the crate in your rust source file by calling `openweathermap::init()` which returns a receiver object which you can then use to call `openweathermap::update()` to get weather updates like in the following example:

```rust
extern crate openweathermap;

fn main() {
    // start our observatory via OWM
    let receiver = &openweathermap::init(city, units, lang, apikey);
    loop {
        match openweathermap::update(receiver) {
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

`openweathermap::init()` will spawn a thread which asks OpenWeatherMap for the current weather every 10 minutes.
Whenever there is an update you get will get it from `openweathermap::update()`.
Within the 10 minutes waiting time you will get `None` which tells you that there is no new update available (see the outer `match` statement in the above example.
You may get an `Err` object if an error has occurred.
Initially while waiting for the first update you will get an `Err` that includes the String "loading..." but also http or json errors may occur.
For example if you use an invalid API key you will get `401 Unauthorized`.
