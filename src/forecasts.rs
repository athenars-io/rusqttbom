use reqwest::{self, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Forecasts {
    data: Today,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Days {
//     o: Today,
//     // _1: Second,
//     // _3: Third,
//     // _4: Fourth,
//     // _5: Fifth,
//     // _6: Sixth,
//     // _7: Seventh,
//     //tomorrow: Tomorrow,
// }

#[derive(Serialize, Deserialize, Debug)]
struct Today {
    rain: Option<Rain>,
    // uv: Option<Uv>,
    // astronomical: Option<Astronomical>,
    // temp_max: Option<f32>,
    // temp_min: Option<f32>,
    // extended_text: Option<String>,
    // short_text: Option<String>,
    // surf_danger: Option<String>,
    // fire_danger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Rain {
    chance: Option<f32>,
    // amount: Option<Amount>,
}

// #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
// struct Amount {
//     min: Option<f32>,
//     max: Option<f32>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Uv {
//     category: Option<String>,
//     max_index: Option<f32>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Astronomical {
//     sunrise_time: Option<String>,
//     sunset_time: Option<String>,
// }

// impl Forecasts {
// Methods for returning forecast data, if it exists. Returns Some or None.
// fn get_rain_chance(&self) -> Option<f32> {
//     self.data.o.rain?.chance
// }

// fn get_f_rain_min(&self) -> Option<f32> {
//     self.data.today.rain?.amount?.min
// }

// fn get_f_rain_max(&self) -> Option<f32> {
//     self.data.today.rain?.amount?.max
// }

// fn get_uv_cat(&self) -> Option<String> {
//     self.data.today.uv?.category
// }

// fn get_uv_index(&self) -> Option<f32> {
//     self.data.today.uv?.max_index
// }

// fn get_sunrise(&self) -> Option<String> {
//     self.data.today.astronomical?.sunrise_time
// }

// fn get_sunset(&self) -> Option<String> {
//     self.data.today.astronomical?.sunset_time
// }
// }

pub async fn get_forecasts() -> Result<(), Box<dyn Error>> {
    let loc_hash = rusqttbom::get_config().location.hash;
    let url = format!("https://api.weather.bom.gov.au/v1/locations/{loc_hash}/forecasts/daily");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        // .text()
        .json::<Forecasts>()
        .await?;

    println!("Response is {:?}", response);

    // Publish the data as MQTT messages
    // if let Some(rain_chanc) = response.get_rain_chance() {
    //     println!("first line");
    //     //if rusqttbom::valid_rain(rain_chanc) {
    //     println!("second line");
    //     let mut rain_c_string = String::new();
    //     rain_c_string = rain_chanc.to_string();
    //     println!("rain c string is {}", rain_c_string);
    //     let rain_c_topic = rusqttbom::get_config().topics.rainchance;
    //     rusqttbom::send_mqtt(rain_c_topic, rain_c_string).await?;
    // }
    Ok(())
}
