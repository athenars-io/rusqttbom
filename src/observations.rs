use reqwest::{self, header::CONTENT_TYPE};
use std::{error::Error, fs};
use serde::{Deserialize, Serialize};
use toml;
use rumqttc::v5::mqttbytes::QoS; // LastWill
use rumqttc::v5::{MqttOptions, AsyncClient};
use std::time::Duration;
use std::env::var;
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
    // port: u16,
}

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

fn get_config_path() -> String {
    // Your config.toml should be in $HOME/.config/rusqttbom/config.toml
    // First, we need to identify the home drive
    // Then we can add on the remainder of the file path
    let home_dir = var("HOME");
    // We need to clean the start and end of this string
    println!("short path {:?}", home_dir);
    // We need to convert the type to String
    let hd_string = format!(
        "{:?}",home_dir 
    );
    let start = 5; // This is to remove formatting from start of file path
    let length = hd_string.len(); 
    let end = length-2; // This is to remove same from end
    let clean_dir = &hd_string[start..end];
    // Now that we have a clean string we can amend the rest
    let config_path = format!(
        "/{}/.config/rusqttbom/config.toml", clean_dir 
    );
    config_path
}

// Result<APIData, Box<dyn Error>> // saving this result type in case I need it again, likely wont need
pub async fn get_weather() -> Result<(), Box<dyn Error>>  {
    // Now we can start to read the config file
    // Optionally, you may want to replace `get_config_path()` with simply "config.toml" for development
    // However setting it this way allows for the binary to run, including by CRON
    let config: Config = {
        let config_text = fs::read_to_string(get_config_path())
            .expect("Could not read the file");
        toml::from_str(&config_text)
            .expect("Could not parse toml")
    };
    // let loc_name = config.location.name;
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
    
    // Setup MQTT client to be ready to send messages
    let ip = config.broker.ip;
    let mut mqttoptions = MqttOptions::new("rusqttbom", ip, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Publish the data as MQTT messages
    // All messages are published with QoS 0
    let temp_c_topic = "outside/weather/current-temp";
    let current_temp = &response.data.temp;
    match &current_temp {
        Some(current_temp) => client
                                        .publish(temp_c_topic, QoS::AtMostOnce, false, current_temp.to_string())
                                        .await
                                        .unwrap(),
        None => println!("None value for current temp"),
    }

    let temp_feels_topic = "outside/weather/temp-feels"; 
    let temp_feels = &response.data.temp_feels_like;
    match &temp_feels {
        Some(temp_feels) => client
                                      .publish(temp_feels_topic, QoS::AtMostOnce, false, temp_feels.to_string())
                                      .await
                                      .unwrap(),
        None => println!("None value for temp feels"),
    }

    let min_temp_topic = "outside/weather/min-temp";
    let min_temp = &response.data.min_temp.value;
    match &min_temp {
        Some(min_temp) => client
                                    .publish(min_temp_topic, QoS::AtMostOnce, false, min_temp.to_string())
                                    .await
                                    .unwrap(),
        None => println!("None value for min temp"),
    }

    let max_temp_topic = "outside/weather/max-temp";
    let max_temp = &response.data.max_temp.value;
    match &max_temp {
        Some(max_temp) => client
                                    .publish(max_temp_topic, QoS::AtMostOnce, false, max_temp.to_string())
                                    .await
                                    .unwrap(),
        None => println!("None value for max temp"),
    }

    let humidity_topic = "outside/weather/humidity";
    let humidity = &response.data.humidity;
    match &humidity {
        Some(humidity) => client
                                    .publish(humidity_topic, QoS::AtMostOnce, false, humidity.to_string())
                                    .await
                                    .unwrap(),
        None => println!("None value for humidity"),
    }

    let rain_today_topic = "outside/weather/rain-today";
    let rain_today = &response.data.rain_since_9am;
    match &rain_today {
        Some(rain_today) => client
                                      .publish(rain_today_topic, QoS::AtMostOnce, false, rain_today.to_string())
                                      .await
                                      .unwrap(),
        None => println!("None value for rain today"),
    }

    let wind_km_topic = "outside/weather/wind-kms";
    let wind_kms = &response.data.wind.speed_kilometre;
    match &wind_kms {
        Some(wind_kms) => client
                                    .publish(wind_km_topic, QoS::AtMostOnce, false, wind_kms.to_string())
                                    .await
                                    .unwrap(),
        None => println!("None value for wind kms"),
    }

    let wind_kts_topic = "outside/weather/wind-kts";
    let wind_kts = &response.data.wind.speed_knot;
    match &wind_kts {
        Some(wind_kts) => client
                                    .publish(wind_kts_topic, QoS::AtMostOnce, false, wind_kts.to_string())
                                    .await
                                    .unwrap(),
        None => println!("None value for wind kts"),
    }

    let wind_dir_topic = "outside/weather/wind-dir";
    let wind_direction = &response.data.wind.direction;
    match &wind_direction {
        Some(wind_direction) => client
                                             .publish(wind_dir_topic, QoS::AtMostOnce, false, wind_direction.to_string())
                                             .await
                                             .unwrap(),
        None => println!("None value for wind direction"),
    }

    let gusts_topic = "outside/weather/gusts";
    let gusts = &response.data.gust;
    match &gusts {
        Some(gusts) => client
                                 .publish(gusts_topic, QoS::AtMostOnce, false, gusts.to_string())
                                 .await
                                 .unwrap(),
        None => println!("None value for gusts"), // this can later be a log message
    }

    let max_gust_topic = "outside/weather/max-gust";
    let max_gust = &response.data.max_gust;
     // Later, when sending MQTT messages, we can just send when the pattern matches    
     match &max_gust {
        Some(max_gust) => client
                                    .publish(max_gust_topic, QoS::AtMostOnce, false, max_gust.to_string())
                                    .await
                                    .unwrap(),
        None => println!("None value for max gusts"), // this can later be a log message
    }

   // This endless eventloop is required to publish the messages
   // The count needs to give enough times to receive the ConnAck and complete this program and wait for any failed messages to process
   let mut n = 15;    
   loop {
       let event = eventloop.poll().await;
       n -= 1; // this countdown allows us to break out of the endless loop closing the program
       match &event {
           Ok(v) => {
               if n < 1 {
                   println!("breaking out");
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

    // The below works well and correct
    // We can build on the below later for error checking etc.
    //match &response.json::<APIData>().await {
    //    Ok(parsed) => println!("Success! {:?}", parsed),
    //    Err(_) => println!("didn't parse"),
    //};
    // Ok(response)
}