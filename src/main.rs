use reqwest::{self, header::CONTENT_TYPE};
use std::{error::Error, fs};
use serde::{Deserialize, Serialize};
use toml;
use rumqttc::v5::mqttbytes::{LastWill, QoS};
use rumqttc::v5::{MqttOptions, AsyncClient};
use std::time::Duration;
use std::thread;
use tokio::{task, time};
//use serde_json;

#[derive(Deserialize, Debug)]
struct Config {
    location: Location,
    broker: Broker,
}

#[derive(Deserialize, Debug)]
struct Location {
    name: String,
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
    
    // Setup MQTT client to be ready to send messages
    let ip = config.broker.ip;
    // println!("ip is {}", ip);
    let port = config.broker.port;
    // println!("port is {}", port);
    //let mut mqttoptions = MqttOptions::new("RusQTTbom", "192.168.1.44", 1883);
    //mqttoptions.set_keep_alive(Duration::from_secs(5));
    //let (client, mut connection) = Client::new(mqttoptions, 10);
    //thread::spawn(move || publish(clientz, "test"));
    //client.subscribe("rusqttbom/log", QoS::AtMostOnce).unwrap();
    //publish(&client);
    //thread::spawn(move || publish(client));
    //    thread::sleep(Duration::from_millis(100));
    
    //clientz.publish("rusqttbom/log", QoS::AtMostOnce, false, "test")
    //        .expect("could not send message");
    //clientz.publish("rusqttbom/log", QoS::AtMostOnce, false, "test").unwrap();

    // The following variables can later be published via
    // MQTT messages along their own MQTT topics.
    // At that point, we will no longer need the println! lines
    // as those lines are temporary only.
    
    let current_temp = &response.data.temp;
    match &current_temp {
        Some(current_temp) => println!("The current temperature at {} is {:?} degrees", loc_name, &current_temp),
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

async fn send_mqtt() -> Result<(), Box<dyn Error>> {
    let mut mqttoptions = MqttOptions::new("mqtt-play", "192.168.1.44", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    let topicz = "mqtt/more";
    let payloadz = 22.7;
    let new = payloadz.to_string();
    // This below publish now works, though need to convert float to string
    client
        .publish(topicz, QoS::AtMostOnce, false, new)
        .await
        .unwrap();

    loop {
        let event = eventloop.poll().await;
        match &event {
            Ok(v) => {
                println!("Event = {:?}", v);
            }
            Err(e) => {
                println!("Error = {:?}", e);
                return Ok(());
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let detail = get_weather().await;
    println!("weather = {:#?}", detail);

    send_mqtt().await;
}
    //let mut mqttoptions = MqttOptions::new("mqtt-play", "192.168.1.44", 1883);
    //mqttoptions.set_keep_alive(Duration::from_secs(5));
    //let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    //let topicz = "mqtt/more";
    //let payloadz = "testing above";
    //client
    //    .publish(topicz, QoS::ExactlyOnce, false, payloadz)
    //    .await
    //    .unwrap();
    //task::spawn(async move {
    //    requests(client).await;
    //    time::sleep(Duration::from_secs(2)).await;
    //});


//async fn requests(client: AsyncClient) {
    //client
    //    .subscribe("mqtt-play/testing", QoS::AtMostOnce)
    //    .await
    //    .unwrap();

//    client
  //      .publish("mqtt-play/testing", QoS::ExactlyOnce, false, "testing new")
    //    .await
    //    .unwrap();

//    time::sleep(Duration::from_secs(1)).await;
//}
