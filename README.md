# RusQTTbom 

RusQTTbom collects weather data from the Bureau of Meterology (BOM) then publishes said data locally via MQTT messages. BOM weather data is only for Australian locations. This weather data is obtained via an undocumented API, which was seemingly discovered, or at least popularised, thanks to [this Github repo](https://github.com/chris-horner/SocketWeather) - and [this one](https://github.com/bremor/bureau_of_meteorology). Because this is an undocumented API, it could be turned off at any time. BOM weather data is generally accepted as being the most accurate weather data in Australia.

The main idea with this program is to collect the BOM weather data via the API, parse the data, check for null or erroneous entries, then publish the wanted data in a local network via MQTT messages. This way, we can have many programs do things with the MQTT data in a de-coupled manner. For example, we can setup alerts and notifications. We can use various home automations. We can also write the data to a database. By using an MQTT message bus, we can loosely couple all of these services, which provides desired reliability in our system. In essense, RusQTTbom becomes a local weather sensor providing BOM weather data.

The BOM weather data API is used in [this BOM weather site](https://weather.bom.gov.au/). Go to this website then enter your suburb / location in the search bar then click on the correct result. Take a note of the seven digit *geohash* in the URL in the address bar of your browser. This will be used to get data for your location and you need to add this to your `config.toml` file.

<<<<<<< HEAD
RusQTTbom publishes weather observations for the current day, as well as forecasts for the current day and the next day. Forecasts for seven days will be coming in future releases.

Weather data that is published by RusQTTbom via MQTT consists of the following values, by the MQTT topics:
=======
This program is very stable and works well as is. I've personnaly been using this for many months and have never had one issue with it. The only issues come from the missing entries from the API itself. Sometimes, some values are simply not published from the source. Regardless, RusQTTbom will collect and validate whatever data has been publised, and the data as MQTT messages. So while there may not be much maintainance, this does not mean it this project is not maintained. It just works. Other features will come in due course.

Weather data *observations* are published by RusQTTbom via MQTT and consist of the following values, by the MQTT topics:
>>>>>>> forecasts

- outside/weather/current-temp
- outside/weather/temp-feels
- outside/weather/min-temp
- outside/weather/max-temp
- outside/weather/humidity
- outside/weather/rain-today
- outside/weather/wind-kms
- outside/weather/wind-dir
- outside/weather/gusts-kms
- outside/weather/max-gust
- outside/forecast-0/rain-chance
- outside/forecast-0/rain-min
- outside/forecast-0/rain-max
- outside/forecast-0/temp-min
- outside/forecast-0/temp-max
- outside/forecast-0/sunrise
- outside/forecast-0/sunset
- outside/forecast-0/extended
- outside/forecast-0/short
- outside/forecast-0/uv-category
- outside/forecast-0/uv-index
- outside/forecast-1/rain-chance
- outside/forecast-1/rain-min
- outside/forecast-1/rain-max
- outside/forecast-1/temp-min
- outside/forecast-1/temp-max
- outside/forecast-1/sunrise
- outside/forecast-1/sunset
- outside/forecast-1/extended
- outside/forecast-1/short
- outside/forecast-1/uv-category
- outside/forecast-1/uv-index

Weather data *forecasts* for the current day and the next day are also published by RusQTTbom via MQTT and consists of the following values, by MQTT topics:

Same day
- outside/forecast-0/rain-chance
- outside/forecast-0/rain-min
- outside/forecast-0/rain-max
- outside/forecast-0/temp-min
- outside/forecast-0/temp-max
- outside/forecast-0/sunrise
- outside/forecast-0/sunset
- outside/forecast-0/extended
- outside/forecast-0/short 
- outside/forecast-0/uv-category
- outside/forecast-0/uv-index
- outside/forecast-0/firedanger

Next day
- outside/forecast-1/rain-chance
- outside/forecast-1/rain-min
- outside/forecast-1/rain-max
- outside/forecast-1/temp-min
- outside/forecast-1/temp-max
- outside/forecast-1/sunrise
- outside/forecast-1/sunset
- outside/forecast-1/extended
- outside/forecast-1/short 
- outside/forecast-1/uv-category
- outside/forecast-1/uv-index
- outside/forecast-1/firedanger

All of the above MQTT topic names are user configurable (via the `config.toml` file). Other user configuration options consist of the following:

- Location name
- Location geohash
- MQTT broker IP address
- MQTT broker port
- Minimum valid temperature
- Maximum valid temperature
- Minimum valid wind speed
- Maximum valid wind speed
- Minimum valid humidity
- Maximum valid humidity
- Minimum valid rainfall
- Maximum valid rainfall

The data validation settings are to catch erroneous entries such as -99 or 999 and the like. RusQTTbom publishes the weather data asyncronously, which is totally unnecessary but it does. The RusQTTbom MQTT client name is 'rusqttbom'

RusQTTbom expects the `config.toml` file to be saved under the users home directory in `/.config/rusqttbom/config.toml`. The program will not create the directory or move the config file there. A user needs to do this, however it only needs to be done once. This works MacOS and it *should* work on other OSs too. Let me know if there are issues.

Issues can be viewed [here](https://github.com/athenars-io/rusqttbom/issues), and the development plan can be found at the [Project](https://github.com/orgs/athenars-io/projects/1/views/2).

To use this program in its current state, the repo can be forked and cloned, configuration file edited and saved in the appropriate directory (as explained), then use `cargo build --release`. The binary itself, called `rusqttbom`, can be located at `rusqttbom/target/release`. The binary itself can be copied or moved anywhere else in your file system and can run by itself. Just remember the `config.toml` file. For best use of this program, be sure to set a cron job to run the binary every 10mins. Here is an example cron job that will run this program every 10mins.

```shell
crontab -e
*/10 * * * * /full/file/path/to/binary/rusqttbom
```

Once you get the data flowing into your environment via MQTT (via the cron jobs), you can setup automations, notifications, alerts, graph the data and / or save it to a database for longer term storage and analysis. 

**Important!**: Do not hit this API more than once every ten minutes. Not abusing this API should help keep it from being shut down or modified. The API seems to only be updated every 10mins at a minimum so keeping your calls to a minimum makes sense.

Be sure to star this repo to watch the progress if this interests you!