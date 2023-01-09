use reqwest::{self, header::CONTENT_TYPE};
use std::{error::Error};
use serde::{Deserialize, Serialize};
//use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct APIData {
    data: Weather,
    //metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    // Note all of the Options are dealt with in the get_weather function
    // by identifying null / None values then processing accordingly
    temp: Option<f32>,
    temp_feels_like: Option<f32>,
    min_temp: MinTemp,
    max_temp: MaxTemp,
    humidity: Option<f32>,
    rain_since_9am: Option<f32>,
    wind: Wind,
    gust: Option<f32>, 
    #[serde(skip_serializing_if = "Option::is_none")] // Do I need this?
    max_gust: Option<f32>, 
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed_knot: Option<f32>,
    speed_kilometre: Option<f32>,
    direction: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MaxTemp {
    value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MinTemp {
    value: Option<f32>,
}

async fn get_weather() -> Result<APIData, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.weather.bom.gov.au/v1/locations/r3gx2f/observations")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<APIData>() // later will move this into a match function
        .await?;

    // the following variables can later be published via
    // MQTT messages along their own MQTT topics.
    // At that point, we will no longer need the println! lines
    // as those lines are temporary only.

    let current_temp = &response.data.temp;
    match &current_temp {
        Some(current_temp) => println!("The current temperature is {:?} degrees", &current_temp),
        None => println!("None value for current temp"),
    }

    let temp_feels = &response.data.temp_feels_like;
    match &temp_feels {
        Some(temp_feels) => println!("The current temperature feels like {:?} degrees", &temp_feels),
        None => println!("None value for temp feels"),
    }

    let min_temp = &response.data.min_temp.value;
    match &min_temp {
        Some(min_temp) => println!("The minimum temperature today has been {:?} degrees", &min_temp),
        None => println!("None value for min temp"),
    }

    let max_temp = &response.data.max_temp.value;
    match &max_temp {
        Some(max_temp) => println!("The maximum temperature today has been {:?} degrees", &max_temp),
        None => println!("None value for max temp"),
    }

    let humidity = &response.data.humidity;
    match &humidity {
        Some(humidity) => println!("The current humidity is {:?} percent", &humidity),
        None => println!("None value for humidity"),
    }

    let rain_today = &response.data.rain_since_9am;
    match &rain_today {
        Some(rain_today) => println!("Today, there has been {:?}mm of rain", &rain_today),
        None => println!("None value for rain today"),
    }

    let wind_kms = &response.data.wind.speed_kilometre;
    match &wind_kms {
        Some(wind_kms) => println!("The wind speed is {:?}km/h", &wind_kms),
        None => println!("None value for wind kms"),
    }

    let wind_kts = &response.data.wind.speed_knot;
    match &wind_kts {
        Some(wind_kts) => println!("the wind speed is {:?}knots", &wind_kts),
        None => println!("None value for wind kts"),
    }

    let wind_direction = &response.data.wind.direction;
    match &wind_direction {
        Some(wind_direction) => println!("The wind is coming from the {:?} direction", &wind_direction),
        None => println!("None value for wind direction"),
    }

    let gusts = &response.data.gust;
    // Later, when sending MQTT messages, we can just send when the pattern matches 
    match &gusts {
        Some(gusts) => println!("The wind is gusting at {:?}km an hour", &gusts),
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
