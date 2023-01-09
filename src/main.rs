use reqwest::{self, header::CONTENT_TYPE};
use std::error::Error;
use serde::{Deserialize, Serialize};
//use serde_json;

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
    // gust: String, // how to manage null values?
    // max_gust: String // how to manage null values?
    wind: Wind,
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
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.weather.bom.gov.au/v1/locations/r3gx2f/observations")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<APIData>()
        .await?;

    // the following variables can later be published via
    // MQTT messages along their own MQTT topics.
    // At that point, we will no longer need the println! lines
    // as those lines are temporary only.

    let current_temp = &response.data.temp;
    println!("The current temperature is {:?} degrees", &current_temp);

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
