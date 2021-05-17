# openweathermap

This is a crate which lets you easily access current weather data from [OpenWeatherMap](https://openweathermap.org/).

## How to use

First add this crate to your dependencies in you `Cargo.toml` file:

```toml
[dependencies]
openweathermap = "0.0.8"
```

Then use the crate in your rust source file by calling `openweathermap::init()` which returns a receiver object which you can then use to call `openweathermap::update()` to get weather updates like in the following example:

```rust
extern crate openweathermap;

fn main() {
    // start our observatory via OWM
    let receiver = &openweathermap::init(
        "Berlin,DE",
        "metric",
        "en",
        "<your OpenWeatherMap API key>",
        10,
    );
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

`openweathermap::init()` will spawn a thread which asks OpenWeatherMap for the current weather periodically.
Whenever there is an update you can get it by using `openweathermap::update()`.

Within the polling period you might get `None` which tells you that there is no new update available (see the outer `match` statement in the above example).

You may get an `Err` object if an error occurs.
Initially while waiting for the first update you will get an `Err` that includes the String "loading..." but also http or json errors may occur.
For example if you use an invalid API key you will get `401 Unauthorized`.

### Reference

#### openweathermap::init()

Spawns a thread which fetches the current weather from [openweathermap.org](https://openweathermap.org) periodically.

##### Definition:

`pub fn init(location: &str, units: &str, lang: &str, api_key: &str, poll_mins: u64) -> Receiver`

##### Parameters:

-   `location` : Can be a city name, a city ID or a geographical coordinate. The city name may be followed by comma separated state code and/or country code (e.g. `"Berlin,DE"`). The city ID can be found using [this](https://openweathermap.org/find) website. There you can search for your city which will give you the ID from the link you get. Coordinates are given by comma separated latitude and longitude (e.g. `"52.5244,13.4105"`).
-   `units` : Either `"metric"` (meters, m/s, °C, etc.), `"imperial"` (miles, mi/s, °F, etc.) or `"standard"` (meters, m/s, K, etc.)
-   `lang` : Language code line `"en"` for English (see [this list](https://openweathermap.org/current#multi) for all available language codes).
-   `api_key` : Your API key you can get from [OpenWeatherMap](https://openweathermap.org/price).
-   `poll_mins` : Poll period length in minutes (`10` is recommended). If `poll_mins` equals `0` the thread will terminate after the first successful update.

##### Return Value:

Returns the receiver object which you need to get the latest weather update from `openweathermap::update()`.

#### openweathermap::update()

Get the latest weather update that the spawned thread has fetched.

##### Definition:

`pub fn update(receiver: &Receiver) -> Option<Result<CurrentWeather,String>>`

##### Parameters:

- `receiver` : The receiver object you previously got from `openweathermap::init()`.

##### Return Value:

Returns a `Option<Result<CurrentWeather,String>>` which is `None` if  currently there is no new weather update available.

Otherwise could be `Some<CurrentWeather>` on success or `Err<String>` if an error has occurred. The error could be about a http or json issue. For example you will get `401 Unauthorized` if your API key is invalid or some json parser error message if there is something wrong with the response from OpenWeatherMap.

On success you get a `CurrentWeather` object which is a nested struct including all weather properties which are provided. Those properties are well described [here](https://openweathermap.org/current#parameter).
All property names are like in this description except `sys.type_` which has a `_` appended so that it does not collide with the rust keyword `type`.
