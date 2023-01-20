use reqwest::{self, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct APIData {
    data: Weather,
}

#[derive(Serialize, Deserialize, Debug)]
struct Weather {
    temp: Option<f32>,
    temp_feels_like: Option<f32>,
    min_temp: Option<MinTemp>,
    max_temp: Option<MaxTemp>,
    humidity: Option<f32>,
    rain_since_9am: Option<f32>,
    wind: Wind,
    gust: Option<Gusts>,
    max_gust: Option<MaxGusts>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Wind {
    speed_kilometre: Option<f32>,
    direction: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Gusts {
    speed_kilometre: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct MaxGusts {
    speed_kilometre: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct MaxTemp {
    value: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct MinTemp {
    value: Option<f32>,
}

impl APIData {
    // Methods for returning weather data, if it exists. Returns Some or None.
    fn get_temp(&self) -> Option<f32> {
        self.data.temp
    }

    fn get_temp_feels(&self) -> Option<f32> {
        self.data.temp_feels_like
    }

    fn get_min_temp(&self) -> Option<f32> {
        self.data.min_temp?.value
    }

    fn get_max_temp(&self) -> Option<f32> {
        self.data.max_temp?.value
    }

    fn get_humidity(&self) -> Option<f32> {
        self.data.humidity
    }

    fn get_rain(&self) -> Option<f32> {
        self.data.rain_since_9am
    }

    fn get_gusts(&self) -> Option<f32> {
        self.data.gust?.speed_kilometre
    }

    fn get_gusts_max(&self) -> Option<f32> {
        self.data.max_gust?.speed_kilometre
    }
}

pub async fn get_observations() -> Result<(), Box<dyn Error>> {
    let loc_hash = rusqttbom::get_config().location.hash;
    let url = format!("https://api.weather.bom.gov.au/v1/locations/{loc_hash}/observations");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<APIData>() // this is where the data is sent to the top level struct
        .await?;

    // Publish the data as MQTT messages
    if let Some(temppp) = response.get_temp() {
        if rusqttbom::valid_temp(temppp) {
            let mut temp_string = String::new();
            temp_string = temppp.to_string();
            let temp_c_topic = "outside/weather/current-temp";
            rusqttbom::send_mqtt(temp_c_topic, temp_string).await?;
        }
    }

    if let Some(temp_feels) = response.get_temp_feels() {
        if rusqttbom::valid_temp(temp_feels) {
            let mut temp_feels_string = String::new();
            temp_feels_string = temp_feels.to_string();
            let temp_feels_topic = "outside/weather/temp-feels";
            rusqttbom::send_mqtt(temp_feels_topic, temp_feels_string).await?;
        }
    }

    if let Some(min_temppp) = response.get_min_temp() {
        if rusqttbom::valid_temp(min_temppp) {
            let mut min_temp_string = String::new();
            min_temp_string = min_temppp.to_string();
            let min_temp_topic = "outside/weather/min-temp";
            rusqttbom::send_mqtt(min_temp_topic, min_temp_string).await?;
        }
    }

    if let Some(max_temppp) = response.get_max_temp() {
        if rusqttbom::valid_temp(max_temppp) {
            let mut max_temp_string = String::new();
            max_temp_string = max_temppp.to_string();
            let max_temp_topic = "outside/weather/max-temp";
            rusqttbom::send_mqtt(max_temp_topic, max_temp_string).await?;
        }
    }

    if let Some(humidityyy) = response.get_humidity() {
        if rusqttbom::valid_humidity(humidityyy) {
            let mut humidity_string = String::new();
            humidity_string = humidityyy.to_string();
            let humidity_topic = "outside/weather/humidity";
            rusqttbom::send_mqtt(humidity_topic, humidity_string).await?;
        }
    }

    if let Some(rainnn) = response.get_rain() {
        if rusqttbom::valid_rain(rainnn) {
            let mut rain_string = String::new();
            rain_string = rainnn.to_string();
            let rain_today_topic = "outside/weather/rain-today";
            rusqttbom::send_mqtt(rain_today_topic, rain_string).await?;
        }
    }

    if let Some(windkm) = response.data.wind.speed_kilometre {
        if rusqttbom::valid_wind(windkm) {
            let mut windstring = String::new();
            windstring = windkm.to_string();
            let wind_km_topic = "outside/weather/wind-kms";
            rusqttbom::send_mqtt(wind_km_topic, windstring).await?;
        }
    }

    if let Some(winddir) = &response.data.wind.direction {
        let mut wind_dir_string = String::new();
        wind_dir_string = winddir.to_string();
        let wind_dir_topic = "outside/weather/wind-dir";
        rusqttbom::send_mqtt(wind_dir_topic, wind_dir_string).await?;
    }

    if let Some(guggg) = response.get_gusts() {
        if rusqttbom::valid_wind(guggg) {
            let mut gust_string = String::new();
            gust_string = guggg.to_string();
            let gusts_topic = "outside/weather/gusts-kms";
            rusqttbom::send_mqtt(gusts_topic, gust_string).await?;
        }
    }

    if let Some(maxw) = response.get_gusts_max() {
        if rusqttbom::valid_wind(maxw) {
            let mut max_wind_string = String::new();
            max_wind_string = maxw.to_string();
            let max_gust_topic = "outside/weather/max-gust";
            rusqttbom::send_mqtt(max_gust_topic, max_wind_string).await?;
        }
    }
    Ok(())
}
