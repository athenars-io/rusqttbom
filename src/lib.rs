use std::env::var;
use std::{error::Error, fs};
use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, MqttOptions};
use std::time::Duration;
use tokio::task;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    // location: Location,
    broker: Broker,
}

#[derive(Deserialize, Debug)]
struct Broker {
    ip: String,
    port: u16,
}

pub fn get_config_path() -> String {
    // Your config.toml should be in $HOME/.config/rusqttbom/config.toml
    // First, we need to identify the home drive
    // Then we can add on the remainder of the file path
    let home_dir = var("HOME");
    // We need to clean the start and end of this string
    println!("short path {:?}", home_dir);
    // We need to convert the type to String
    let hd_string = format!("{:?}", home_dir);
    let start = 5; // This is to remove formatting from start of file path
    let length = hd_string.len();
    let end = length - 2; // This is to remove same from end
    let clean_dir = &hd_string[start..end];
    // Now that we have a clean string we can amend the rest
    let config_path = format!("/{}/.config/rusqttbom/config.toml", clean_dir);
    config_path
}

pub async fn send_mqtt(topicz: &str, payloadz: String) -> Result<(), Box<dyn Error>> {
    let config: Config = {
        let config_text =
            fs::read_to_string(get_config_path()).expect("Could not read the file");
        toml::from_str(&config_text).expect("Could not parse toml")
    };

    let ip = config.broker.ip;
    let port = config.broker.port;
    let mut mqttoptions = MqttOptions::new("rusqttbom", ip, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let topiczz = topicz.to_string();
    task::spawn(async move {
        sendit(client.clone(), topiczz, payloadz).await;
        client.disconnect().await.expect("Could not disconnect");
        // time::sleep(Duration::from_secs(3)).await;
    });

    // This endless eventloop is required to publish the messages
    // The count gives enough time to deal with connection issues,
    // however, each task should disconnect deliberately after publishing
    let mut n = 5;
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

async fn sendit(client: AsyncClient, topicz: String, payloadz: String) {
    client
        .publish(topicz, QoS::AtMostOnce, false, payloadz)
        .await
        .unwrap();
}
