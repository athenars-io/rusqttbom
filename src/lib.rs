use rumqttc::v5::mqttbytes::QoS;
use rumqttc::v5::{AsyncClient, MqttOptions};
use serde::Deserialize;
use std::env::var;
use std::time::Duration;
use std::{error::Error, fs};
use tokio::task;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub location: Location,
    broker: Broker,
    validation: Validation,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub hash: String,
    // name: String,
}

#[derive(Deserialize, Debug)]
struct Broker {
    ip: String,
    port: u16,
}

#[derive(Deserialize, Debug)]
struct Validation {
    mintemp: f32,
    maxtemp: f32,
    minwind: f32,
    maxwind: f32,
    minhumidity: f32,
    maxhumidity: f32,
    minrain: f32,
    maxrain: f32,
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

pub fn get_config() -> Config {
    let config: Config = {
        let config_text = fs::read_to_string(get_config_path()).expect("Could not read the file");
        toml::from_str(&config_text).expect("Could not parse toml")
    };
    config
}

pub fn valid_temp(value: f32) -> bool {
    let min = get_config().validation.mintemp;
    let max = get_config().validation.maxtemp;
    value >= min && value <= max
}

pub fn valid_wind(value: f32) -> bool {
    let min = get_config().validation.minwind;
    let max = get_config().validation.maxwind;
    value >= min && value <= max
}

pub fn valid_humidity(value: f32) -> bool {
    let min = get_config().validation.minhumidity;
    let max = get_config().validation.maxhumidity;
    value >= min && value <= max
}

pub fn valid_rain(value: f32) -> bool {
    let min = get_config().validation.minrain;
    let max = get_config().validation.maxrain;
    value >= min && value <= max
}

pub async fn send_mqtt(topicz: &str, payloadz: String) -> Result<(), Box<dyn Error>> {
    let ip = get_config().broker.ip;
    let port = get_config().broker.port;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // These first tests use hardcoded values.
    // They will need to be changed if the config.toml has modified validation values.
    fn test_valid_temp() {
        assert_eq!(valid_temp(23.0), true);
    }

    #[test]
    fn test_bad_high_temp() {
        assert_eq!(valid_temp(55.0), false);
    }

    #[test]
    fn test_bad_low_temp() {
        assert_eq!(valid_temp(-21.0), false);
    }

    #[test]
    fn test_valid_wind() {
        assert_eq!(valid_wind(23.0), true);
    }

    #[test]
    fn test_bad_high_wind() {
        assert_eq!(valid_wind(425.0), false);
    }

    #[test]
    fn test_bad_low_wind() {
        assert_eq!(valid_wind(-2.0), false);
    }

    #[test]
    fn test_valid_humidity() {
        assert_eq!(valid_humidity(23.7), true);
    }

    #[test]
    fn test_bad_high_humidity() {
        assert_eq!(valid_humidity(102.0), false);
    }

    #[test]
    fn test_bad_low_humidity() {
        assert_eq!(valid_humidity(-0.2), false);
    }

    #[test]
    fn test_valid_rain() {
        assert_eq!(valid_rain(22.0), true);
    }

    #[test]
    fn test_bad_high_rain() {
        assert_eq!(valid_rain(599.0), false);
    }

    #[test]
    fn test_bad_low_rain() {
        assert_eq!(valid_rain(-2.0), false);
    }

    #[test]
    // I'm not sure if this is a good test of the code.
    fn test_config_file_path() {
        let result = get_config_path();
        assert!(
            result.contains("config.toml"),
            "Config file path did not contain config.toml. Instead it was {}",
            result
        );
    }
}
