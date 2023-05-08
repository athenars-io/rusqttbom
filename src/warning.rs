use reqwest::{self, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Warnings {
    data: Getit,
    metadata: Metaa,
}

#[derive(Serialize, Deserialize, Debug)]
struct Metaa {
    response_timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Getit {
    o: Details,
}

#[derive(Serialize, Deserialize, Debug)]
struct Details {
    id: Option<String>,
    // typee: Option<String>,
    // title: Option<String>,
    short_title: Option<String>,
    state: Option<String>,
    warning_group_type: Option<String>,
    issue_time: Option<String>,
    expiry_time: Option<String>,
    phase: Option<String>,
}

// Fetch the data
pub async fn get_warnings() -> Result<(), Box<dyn Error>> {
    let loc_hash = rusqttbom::get_config().location.hash;
    let url = format!("https://api.weather.bom.gov.au/v1/locations/{loc_hash}/warnings");
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<Warnings>() // Warning is the top level Struct
        .await?;

    if let Some(titlez) = response.data.o.short_title {
        let title_str = titlez.to_string();
        let title_topic = rusqttbom::get_config().topics.title;
        rusqttbom::send_mqtt(title_topic, title_str).await?;
    }

    if let Some(groupz) = response.data.o.warning_group_type {
        let warning_group_str = groupz.to_string();
        let warning_group_topic = rusqttbom::get_config().topics.warning_group;
        rusqttbom::send_mqtt(warning_group_topic, warning_group_str).await?;
    }

    if let Some(phasez) = response.data.o.phase {
        let phase_str = phasez.to_string();
        let phase_topic = rusqttbom::get_config().topics.phase;
        rusqttbom::send_mqtt(phase_topic, phase_str).await?;
    }

    // To do: issue and expiry time. they use zulu time

    Ok(())
}
