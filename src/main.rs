use reqwest::{self, header::CONTENT_TYPE};
use std::error::Error;
//use std::io::Read;
use serde::{Deserialize, Serialize};
//use serde_json;
//use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct APIData {
    data: Weather,
    //metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    temp: f32,
    temp_feels_like: f32,
    humidity: u32,
    rain_since_9am: f32,
    wind: Wind
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed_knot: u32,
}

async fn get_weather() -> Result<APIData, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.weather.bom.gov.au/v1/locations/r3gx2f/observations")
        .header(CONTENT_TYPE, "application/json")
        //.header(ACCEPT, "application/json")
        .send()
        .await?
        .json::<APIData>()
        .await?;
        //.unwrap();

    // The below works!!!!!!!!!!!
    let current_temp = &response.data.temp;
    println!("The current temperature is {:?} degrees", &current_temp);

    let temp_feels = &response.data.temp_feels_like;
    println!("The current temperature feels like {:?} degrees", temp_feels);

    let humidity = &response.data.humidity;
    println!("The current humidty is {:?} percent", humidity);

    let rain_today = &response.data.rain_since_9am;
    println!("Today, there has been {:?}mm of rain", rain_today);


       
    //println!("Got {:?}", &response);
    //println!("response status = {}", &response.status());

    //let body = &response.text().await;
    //println!("Body {:?}", &body);

    //let tt = json::parse(&body).unwrap();

    //let v: Value = serde_json::from_string(&body).unwrap();

    // The below works well and correct
    //match &response.json::<APIData>().await {
    //    Ok(parsed) => println!("Success! {:?}", parsed),
    //    Err(_) => println!("didn't parse"),
    //};

    //println!("issue time is {}", &parsed["issue_time"])

    //let metaz = &response.json::<APIData>().await;

    //for line in metaz {
    //    println!("{:#?}", line);
    //    println!("test {}", line.metadata)
    //}

    //let textr = &response.text().await;

    //println!("to text {:?}", textr);

    Ok(response)
}

#[tokio::main]
async fn main() {
    let detail = get_weather().await;

    println!("weather = {:#?}", detail);
}
