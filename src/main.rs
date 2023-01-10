use reqwest::{self, header::CONTENT_TYPE};
use std::{error::Error, fs};
use serde::{Deserialize, Serialize};
use toml;
//use serde_json;

#[derive(Deserialize, Debug)]
struct Config {
    location: Location
}

#[derive(Deserialize, Debug)]
struct Location {
    name: String,
    hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIData {
    data: Weather,
    //metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    temp: f32,
    temp_feels_like: f32,
    min_temp: MinTemp,
    max_temp: MaxTemp,
    humidity: u32,
    rain_since_9am: f32,
    wind: Wind,
    gust: Option<String>, // These Options are dealt with when saving variables 
    #[serde(skip_serializing_if = "Option::is_none")] // Do I need this?
    max_gust: Option<String>, 
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed_knot: f32,
    speed_kilometre: f32,
    direction: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct MaxTemp {
    value: f32,
}

#[derive(Serialize, Deserialize, Debug)]
struct MinTemp {
    value: f32,
}

async fn get_weather() -> Result<APIData, Box<dyn Error>> {
    // Start by grabbing data from the config.toml file
    let config: Config = {
        let config_text = fs::read_to_string("config.toml")
            .expect("Could not read the file");
        toml::from_str(&config_text)
            .expect("Could not parse toml")
    };
    let loc_name = config.location.name;
    let loc_hash = config.location.hash;
    // println!("location name is {}", &loc_name);
    // println!("location hash is {}", &loc_hash);

    // The following builds the API URL using location data from config.toml
    let url = format!(
        "https://api.weather.bom.gov.au/v1/locations/{loc_hash}/observations"
    );

    // GET the BOM API data
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<APIData>() // later will move this into a match function
        .await?;

    // The following variables can later be published via
    // MQTT messages along their own MQTT topics.
    // At that point, we will no longer need the println! lines
    // as those lines are temporary only.
    let current_temp = &response.data.temp;
    println!("The current temperature at {} is {:?} degrees", &loc_name, &current_temp);

    let temp_feels = &response.data.temp_feels_like;
    println!("The current temperature feels like {:?} degrees", temp_feels);

    let min_temp = &response.data.min_temp.value;
    println!("The minimum temperature today has been {:?} degrees", min_temp);

    let max_temp = &response.data.max_temp.value;
    println!("The maximum temperature today has been {:?} degrees", max_temp);

    let humidity = &response.data.humidity;
    println!("The current humidty is {:?} percent", humidity);

    let rain_today = &response.data.rain_since_9am;
    println!("Today, there has been {:?}mm of rain", rain_today);

    let wind_kms = &response.data.wind.speed_kilometre;
    println!("The wind speed is {:?}km/h", wind_kms);

    let wind_kts = &response.data.wind.speed_knot;
    println!("the wind speed is {:?}knots", wind_kts);

    let wind_direction = &response.data.wind.direction;
    println!("The wind is coming from the {:?} direction", wind_direction);

    let gusts = &response.data.gust;
    // Later, when sending MQTT messages, we can just send when the pattern matches 
    match &gusts {
        Some(gusts) => println!("The wind is gusting at {:?}km an hour", gusts),
        None => println!("None value for gusts"), // this can later be a log message
    }

    let max_gust = &response.data.max_gust;
     // Later, when sending MQTT messages, we can just send when the pattern matches    
     match &max_gust {
        Some(max_gust) => println!("The wind is gusting at up to {:?}km an hour", &max_gust),
        None => println!("None value for max gusts"), // this can later be a log message
    }

    // The below works well and correct
    // We can build on the below later for error checking etc.
    //match &response.json::<APIData>().await {
    //    Ok(parsed) => println!("Success! {:?}", parsed),
    //    Err(_) => println!("didn't parse"),
    //};

    Ok(response)
}

#[tokio::main]
async fn main() {
    let detail = get_weather().await;

    println!("weather = {:#?}", detail);
}
