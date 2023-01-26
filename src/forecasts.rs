use reqwest::{self, header::CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Forecasts {
    data: Days,
    metadata: Meta,
}

#[derive(Serialize, Deserialize, Debug)]
struct Meta {
    forecast_region: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Days {
    o: Today,
    o1: First,
    _2: Second,
    _3: Third,
    _4: Fourth,
    _5: Fifth,
    _6: Sixth,
    // _7: Seventh,
}

#[derive(Serialize, Deserialize, Debug)]
struct Today {
    rain: Option<Rain>,
    uv: Uv,
    astronomical: Astronomical,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    extended_text: Option<String>,
    short_text: Option<String>,
    surf_danger: Option<String>,
    fire_danger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Rain {
    chance: Option<f32>,
    amount: Option<Amount>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Amount {
    min: Option<f32>,
    max: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Uv {
    category: Option<String>,
    max_index: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Astronomical {
    sunrise_time: Option<String>,
    sunset_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct First {
    rain: Option<Rain1>,
    uv: Uv1,
    astronomical: Astronomical1,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    extended_text: Option<String>,
    short_text: Option<String>,
    surf_danger: Option<String>,
    fire_danger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Rain1 {
    chance: Option<f32>,
    amount: Option<Amount1>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Amount1 {
    min: Option<f32>,
    max: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Uv1 {
    category: Option<String>,
    max_index: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Astronomical1 {
    sunrise_time: Option<String>,
    sunset_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Second {
    rain: Option<Rain2>,
    uv: Uv2,
    astronomical: Astronomical2,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    extended_text: Option<String>,
    short_text: Option<String>,
    surf_danger: Option<String>,
    fire_danger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Rain2 {
    chance: Option<f32>,
    // amount: Option<Amount>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Amount2 {
    min: Option<f32>,
    max: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Uv2 {
    category: Option<String>,
    max_index: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Astronomical2 {
    sunrise_time: Option<String>,
    sunset_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Third {
    rain: Option<Rain3>,
    uv: Option<Uv3>,
    astronomical: Option<Astronomical3>,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    extended_text: Option<String>,
    short_text: Option<String>,
    surf_danger: Option<String>,
    fire_danger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Rain3 {
    chance: Option<f32>,
    // amount: Option<Amount>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Amount3 {
    min: Option<f32>,
    max: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Uv3 {
    category: Option<String>,
    max_index: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Astronomical3 {
    sunrise_time: Option<String>,
    sunset_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Fourth {
    rain: Option<Rain4>,
    uv: Option<Uv4>,
    astronomical: Option<Astronomical4>,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    extended_text: Option<String>,
    short_text: Option<String>,
    surf_danger: Option<String>,
    fire_danger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Rain4 {
    chance: Option<f32>,
    // amount: Option<Amount>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Amount4 {
    min: Option<f32>,
    max: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Uv4 {
    category: Option<String>,
    max_index: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Astronomical4 {
    sunrise_time: Option<String>,
    sunset_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Fifth {
    rain: Option<Rain5>,
    uv: Option<Uv5>,
    astronomical: Option<Astronomical5>,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    extended_text: Option<String>,
    short_text: Option<String>,
    surf_danger: Option<String>,
    fire_danger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Rain5 {
    chance: Option<f32>,
    // amount: Option<Amount>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Amount5 {
    min: Option<f32>,
    max: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Uv5 {
    category: Option<String>,
    max_index: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Astronomical5 {
    sunrise_time: Option<String>,
    sunset_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Sixth {
    rain: Option<Rain6>,
    uv: Option<Uv6>,
    astronomical: Option<Astronomical6>,
    temp_max: Option<f32>,
    temp_min: Option<f32>,
    extended_text: Option<String>,
    short_text: Option<String>,
    surf_danger: Option<String>,
    fire_danger: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Rain6 {
    chance: Option<f32>,
    // amount: Option<Amount>,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
struct Amount6 {
    min: Option<f32>,
    max: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Uv6 {
    category: Option<String>,
    max_index: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Astronomical6 {
    sunrise_time: Option<String>,
    sunset_time: Option<String>,
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Seventh {
//     rain: Option<Rain7>,
//     uv: Option<Uv7>,
//     astronomical: Option<Astronomical7>,
//     temp_max: Option<f32>,
//     temp_min: Option<f32>,
//     extended_text: Option<String>,
//     short_text: Option<String>,
//     surf_danger: Option<String>,
//     fire_danger: Option<String>,
// }

// #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
// struct Rain7 {
//     chance: Option<f32>,
//     // amount: Option<Amount>,
// }

// #[derive(Serialize, Deserialize, Debug, Copy, Clone)]
// struct Amount7 {
//     min: Option<f32>,
//     max: Option<f32>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Uv7 {
//     category: Option<String>,
//     max_index: Option<f32>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct Astronomical7 {
//     sunrise_time: Option<String>,
//     sunset_time: Option<String>,
// }

impl Forecasts {
    // Methods for returning forecast data, if it exists. Returns Some or None.
    fn get_rain_chance(&self) -> Option<f32> {
        self.data.o.rain?.chance
    }

    fn get_f_rain_min(&self) -> Option<f32> {
        self.data.o.rain?.amount?.min
    }

    fn get_f_rain_max(&self) -> Option<f32> {
        self.data.o.rain?.amount?.max
    }

    fn get_f_temp_min(&self) -> Option<f32> {
        self.data.o.temp_min
    }

    fn get_f_temp_max(&self) -> Option<f32> {
        self.data.o.temp_max
    }

    // fn get_uv_cat(&self) -> Option<String> {
    //     self.data.o.uv?.category
    // }

    // Cannot use UV index in this manner due to a String value being in Struct
    // fn get_uv_index(&self) -> Option<f32> {
    //     self.data.o.uv?.max_index
    // }

    // fn get_sunrise(&self) -> Option<String> {
    //     self.data.o.astronomical?.sunrise_time
    // }

    // fn get_sunset(&self) -> Option<String> {
    //     self.data.o.astronomical?.sunset_time
    // }

    fn get_rain_chance1(&self) -> Option<f32> {
        self.data.o1.rain?.chance
    }

    fn get_f_rain_min1(&self) -> Option<f32> {
        self.data.o1.rain?.amount?.min
    }

    fn get_f_rain_max1(&self) -> Option<f32> {
        self.data.o1.rain?.amount?.max
    }

    fn get_f_temp_min1(&self) -> Option<f32> {
        self.data.o1.temp_min
    }

    fn get_f_temp_max1(&self) -> Option<f32> {
        self.data.o1.temp_max
    }
}

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

    // Publish the data as MQTT messages

    // 0: Today
    if let Some(rain_chanc) = &response.get_rain_chance() {
        if rusqttbom::valid_rain(rain_chanc) {
            let mut rain_c_string = String::new();
            rain_c_string = rain_chanc.to_string();
            let rain_c_topic = rusqttbom::get_config().topics.rainchance;
            rusqttbom::send_mqtt(rain_c_topic, rain_c_string).await?;
        }
    }

    if let Some(rain_minn) = &response.get_f_rain_min() {
        if rusqttbom::valid_rain(rain_minn) {
            let mut rain_min_string = String::new();
            rain_min_string = rain_minn.to_string();
            let rain_min_topic = rusqttbom::get_config().topics.rainmin;
            rusqttbom::send_mqtt(rain_min_topic, rain_min_string).await?;
        }
    }

    if let Some(rain_maxx) = &response.get_f_rain_max() {
        if rusqttbom::valid_rain(rain_maxx) {
            let mut rain_max_string = String::new();
            rain_max_string = rain_maxx.to_string();
            let rain_max_topic = rusqttbom::get_config().topics.rainmax;
            rusqttbom::send_mqtt(rain_max_topic, rain_max_string).await?;
        }
    }

    if let Some(f_temp_min) = response.get_f_temp_min() {
        if rusqttbom::valid_temp(f_temp_min) {
            let mut ftemp_min_string = String::new();
            ftemp_min_string = f_temp_min.to_string();
            let ftemp_min_topic = rusqttbom::get_config().topics.tempmin;
            rusqttbom::send_mqtt(ftemp_min_topic, ftemp_min_string).await?;
        }
    }

    if let Some(f_temp_max) = response.get_f_temp_max() {
        if rusqttbom::valid_temp(f_temp_max) {
            let mut ftemp_max_string = String::new();
            ftemp_max_string = f_temp_max.to_string();
            let ftemp_max_topic = rusqttbom::get_config().topics.tempmax;
            rusqttbom::send_mqtt(ftemp_max_topic, ftemp_max_string).await?;
        }
    }

    if let Some(sunrise) = &response.data.o.astronomical.sunrise_time {
        let mut sunrise_str = String::new();
        sunrise_str = sunrise.to_string();
        let sunrise_topic = rusqttbom::get_config().topics.sunrise;
        rusqttbom::send_mqtt(sunrise_topic, sunrise_str).await?;
    }

    if let Some(sunset) = &response.data.o.astronomical.sunset_time {
        let mut sunset_str = String::new();
        sunset_str = sunset.to_string();
        let sunset_topic = rusqttbom::get_config().topics.sunset;
        rusqttbom::send_mqtt(sunset_topic, sunset_str).await?;
    }

    if let Some(ext) = &response.data.o.extended_text {
        let mut ext_str = String::new();
        ext_str = ext.to_string();
        let ext_topic = rusqttbom::get_config().topics.extended;
        rusqttbom::send_mqtt(ext_topic, ext_str).await?;
    }

    if let Some(short) = &response.data.o.short_text {
        let mut short_str = String::new();
        short_str = short.to_string();
        let short_topic = rusqttbom::get_config().topics.short;
        rusqttbom::send_mqtt(short_topic, short_str).await?;
    }

    if let Some(uvcat) = &response.data.o.uv.category {
        let mut uvcat_str = String::new();
        uvcat_str = uvcat.to_string();
        let uvcat_topic = rusqttbom::get_config().topics.uvcat;
        rusqttbom::send_mqtt(uvcat_topic, uvcat_str).await?;
    }

    if let Some(uvindex) = response.data.o.uv.max_index {
        let mut uvindex_str = String::new();
        uvindex_str = uvindex.to_string();
        let uvindex_topic = rusqttbom::get_config().topics.uvindex;
        rusqttbom::send_mqtt(uvindex_topic, uvindex_str).await?;
    }

    // 1: Next day
    if let Some(rain_chanc1) = &response.get_rain_chance1() {
        if rusqttbom::valid_rain(rain_chanc1) {
            let mut rain_c_string1 = String::new();
            rain_c_string1 = rain_chanc1.to_string();
            let rain_c_topic1 = rusqttbom::get_config().topics.rainchance1;
            rusqttbom::send_mqtt(rain_c_topic1, rain_c_string1).await?;
        }
    }

    if let Some(rain_minn1) = &response.get_f_rain_min1() {
        if rusqttbom::valid_rain(rain_minn1) {
            let mut rain_min_string1 = String::new();
            rain_min_string1 = rain_minn1.to_string();
            let rain_min_topic1 = rusqttbom::get_config().topics.rainmin1;
            rusqttbom::send_mqtt(rain_min_topic1, rain_min_string1).await?;
        }
    }

    if let Some(rain_maxx1) = &response.get_f_rain_max1() {
        if rusqttbom::valid_rain(rain_maxx1) {
            let mut rain_max_string1 = String::new();
            rain_max_string1 = rain_maxx1.to_string();
            let rain_max_topic1 = rusqttbom::get_config().topics.rainmax1;
            rusqttbom::send_mqtt(rain_max_topic1, rain_max_string1).await?;
        }
    }

    if let Some(f_temp_min1) = response.get_f_temp_min1() {
        if rusqttbom::valid_temp(f_temp_min1) {
            let mut ftemp_min_string1 = String::new();
            ftemp_min_string1 = f_temp_min1.to_string();
            let ftemp_min_topic1 = rusqttbom::get_config().topics.tempmin1;
            rusqttbom::send_mqtt(ftemp_min_topic1, ftemp_min_string1).await?;
        }
    }

    if let Some(f_temp_max1) = response.get_f_temp_max1() {
        if rusqttbom::valid_temp(f_temp_max1) {
            let mut ftemp_max_string1 = String::new();
            ftemp_max_string1 = f_temp_max1.to_string();
            let ftemp_max_topic1 = rusqttbom::get_config().topics.tempmax1;
            rusqttbom::send_mqtt(ftemp_max_topic1, ftemp_max_string1).await?;
        }
    }

    if let Some(sunrise1) = response.data.o1.astronomical.sunrise_time {
        let mut sunrise_str1 = String::new();
        sunrise_str1 = sunrise1.to_string();
        let sunrise_topic1 = rusqttbom::get_config().topics.sunrise1;
        rusqttbom::send_mqtt(sunrise_topic1, sunrise_str1).await?;
    }

    if let Some(sunset1) = response.data.o1.astronomical.sunset_time {
        let mut sunset_str1 = String::new();
        sunset_str1 = sunset1.to_string();
        let sunset_topic1 = rusqttbom::get_config().topics.sunset1;
        rusqttbom::send_mqtt(sunset_topic1, sunset_str1).await?;
    }

    if let Some(ext1) = response.data.o1.extended_text {
        let mut ext_str1 = String::new();
        ext_str1 = ext1.to_string();
        let ext_topic1 = rusqttbom::get_config().topics.extended1;
        rusqttbom::send_mqtt(ext_topic1, ext_str1).await?;
    }

    if let Some(short1) = response.data.o1.short_text {
        let mut short_str1 = String::new();
        short_str1 = short1.to_string();
        let short_topic1 = rusqttbom::get_config().topics.short1;
        rusqttbom::send_mqtt(short_topic1, short_str1).await?;
    }

    if let Some(uvcat1) = response.data.o1.uv.category {
        let mut uvcat_str1 = String::new();
        uvcat_str1 = uvcat1.to_string();
        let uvcat_topic1 = rusqttbom::get_config().topics.uvcat1;
        rusqttbom::send_mqtt(uvcat_topic1, uvcat_str1).await?;
    }

    if let Some(uvindex1) = response.data.o1.uv.max_index {
        let mut uvindex_str1 = String::new();
        uvindex_str1 = uvindex1.to_string();
        let uvindex_topic1 = rusqttbom::get_config().topics.uvindex1;
        rusqttbom::send_mqtt(uvindex_topic1, uvindex_str1).await?;
    }

    Ok(())
}
