use reqwest::{self, header::CONTENT_TYPE};
use rumqttc::v5::mqttbytes::QoS; // LastWill
use rumqttc::v5::{AsyncClient, MqttOptions};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{error::Error, fs};
use toml;
// use std::thread;
// use tokio::{task, time};
// use serde_json;

#[derive(Deserialize, Debug)]
struct Config {
    location: Location,
    broker: Broker,
}

#[derive(Deserialize, Debug)]
struct Location {
    // name: String,
    hash: String,
}

#[derive(Deserialize, Debug)]
struct Broker {
    ip: String,
    port: u16,
}

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
    //speed_knot: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct MaxGusts {
    speed_kilometre: Option<f32>,
    //speed_knot: Option<f32>,
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

    // fn get_wind_km(&self) -> Option<f32> {
    //     self.data.wind?.speed_kilometre
    // }

    // Will need to deal with wind direction later, as cannot copy, clone strings
    // fn get_wind_dir(&self) -> Option<String> {
    //     self.data.wind?.direction
    // }

    fn get_gusts(&self) -> Option<f32> {
        self.data.gust?.speed_kilometre
    }

    fn get_gusts_max(&self) -> Option<f32> {
        self.data.max_gust?.speed_kilometre
    }
}

async fn send_mqtt(topicz: &str, payloadz: &'static str) -> Result<(), Box<dyn Error>> {
    let config: Config = {
        let config_text =
            fs::read_to_string(crate::get_config_path()).expect("Could not read the file");
        toml::from_str(&config_text).expect("Could not parse toml")
    };

    let ip = config.broker.ip;
    let port = config.broker.port;
    let mut mqttoptions = MqttOptions::new("rusqttbom", ip, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    client
        .publish(topicz, QoS::AtMostOnce, false, payloadz)
        .await?;

    // This endless eventloop is required to publish the messages
    // The count needs to give enough times to receive the ConnAck and complete this program and wait for any failed messages to process
    let mut n = 15;
    loop {
        let event = eventloop.poll().await;
        n -= 1; // this countdown allows us to break out of the endless loop closing the program
        match &event {
            Ok(v) => {
                if n < 1 {
                    println!("Breaking out. Program now closing");
                    return Ok(());
                }
                println!("Event = {:?}", v);
            }
            Err(e) => {
                println!("Error = {:?}", e);
                return Ok(());
            }
        }
    }
}

// Result<(), Box<dyn Error>>
pub async fn get_observations() {
    // Optionally, you may want to replace `crate::get_config_path()` with a custom hard coded file path for development
    // Regardless, setting the config file path allows for the binary to run, including by CRON
    let config: Config = {
        let config_text =
            fs::read_to_string(crate::get_config_path()).expect("Could not read the file");
        toml::from_str(&config_text).expect("Could not parse toml")
    };
    // let loc_name = config.location.name;
    let loc_hash = config.location.hash;

    let url = format!("https://api.weather.bom.gov.au/v1/locations/{loc_hash}/observations");

    // GET the BOM API data
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap()
        .json::<APIData>()
        .await;

    send_mqtt("rusqtttest", "test message").await;

    //#################################################

    // // Setup MQTT client to be ready to send messages
    // let ip = config.broker.ip;
    // let port = config.broker.port;
    // let mut mqttoptions = MqttOptions::new("rusqttbom", ip, port);
    // mqttoptions.set_keep_alive(Duration::from_secs(5));
    // let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Publish the data as MQTT messages
    // Calling the methods which return Some or None
    // let temp_c_topic = "outside/weather/current-temp";
    // let mut temp_string = String::new();
    // if let Some(temppp) = response.get_temp() {
    //     temp_string = temppp.to_string();
    // }
    // client
    //     .publish(temp_c_topic, QoS::AtMostOnce, false, temp_string)
    //     .await?;

    // let temp_feels_topic = "outside/weather/temp-feels";
    // let mut temp_feels_string = String::new();
    // if let Some(temp_feels) = response.get_temp_feels() {
    //     temp_feels_string = temp_feels.to_string();
    // }
    // client
    //     .publish(temp_feels_topic, QoS::AtMostOnce, false, temp_feels_string)
    //     .await?;

    // let min_temp_topic = "outside/weather/min-temp";
    // let mut min_temp_string = String::new();
    // if let Some(min_temppp) = response.get_min_temp() {
    //     min_temp_string = min_temppp.to_string();
    // }
    // client
    //     .publish(min_temp_topic, QoS::AtMostOnce, false, min_temp_string)
    //     .await?;

    // let max_temp_topic = "outside/weather/max-temp";
    // let mut max_temp_string = String::new();
    // if let Some(max_temppp) = response.get_max_temp() {
    //     max_temp_string = max_temppp.to_string();
    // }
    // client
    //     .publish(max_temp_topic, QoS::AtMostOnce, false, max_temp_string)
    //     .await?;

    // let humidity_topic = "outside/weather/humidity";
    // let mut humidity_string = String::new();
    // if let Some(humidityyy) = response.get_humidity() {
    //     humidity_string = humidityyy.to_string();
    // }
    // client
    //     .publish(humidity_topic, QoS::AtMostOnce, false, humidity_string)
    //     .await?;

    // let rain_today_topic = "outside/weather/rain-today";
    // let mut rain_string = String::new();
    // if let Some(rainnn) = response.get_rain() {
    //     rain_string = rainnn.to_string();
    // }
    // client
    //     .publish(rain_today_topic, QoS::AtMostOnce, false, rain_string)
    //     .await?;

    // let wind_km_topic = "outside/weather/wind-kms";
    // let mut windstring = String::new();
    // if let Some(windkm) = &response.data.wind.speed_kilometre {
    //     windstring = windkm.to_string();
    // }
    // client
    //     .publish(wind_km_topic, QoS::AtMostOnce, false, windstring)
    //     .await?;

    // let wind_dir_topic = "outside/weather/wind-dir";
    // let mut wind_dir_string = String::new();
    // if let Some(winddir) = &response.data.wind.direction {
    //     wind_dir_string = winddir.to_string();
    // }
    // client
    //     .publish(wind_dir_topic, QoS::AtMostOnce, false, wind_dir_string)
    //     .await?;

    // let gusts_topic = "outside/weather/gusts-kms";
    // let mut gust_string = String::new();
    // if let Some(guggg) = response.get_gusts() {
    //     gust_string = guggg.to_string();
    // }
    // client
    //     .publish(gusts_topic, QoS::AtMostOnce, false, gust_string)
    //     .await?;

    // let max_gust_topic = "outside/weather/max-gust";
    // let mut max_wind_string = String::new();
    // if let Some(maxw) = response.get_gusts_max() {
    //     max_wind_string = maxw.to_string();
    // }
    // client
    //     .publish(max_gust_topic, QoS::AtMostOnce, false, max_wind_string)
    //     .await?;

    // This endless eventloop is required to publish the messages
    // The count needs to give enough times to receive the ConnAck and complete this program and wait for any failed messages to process
    // let mut n = 15;
    // loop {
    //     let event = eventloop.poll().await;
    //     n -= 1; // this countdown allows us to break out of the endless loop closing the program
    //     match &event {
    //         Ok(v) => {
    //             if n < 1 {
    //                 println!("Breaking out. Program now closing");
    //                 return Ok(());
    //             }
    //             println!("Event = {:?}", v);
    //         }
    //         Err(e) => {
    //             println!("Error = {:?}", e);
    //             return Ok(());
    //         }
    //     }
    // }

    // The below works well and correct
    // We can build on the below later for error checking etc.
    //match &response.json::<APIData>().await {
    //    Ok(parsed) => println!("Success! {:?}", parsed),
    //    Err(_) => println!("didn't parse"),
    //};
    // Ok(response)
}
