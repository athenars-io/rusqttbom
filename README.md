# RusQTTbom 

RusQTTbom collects weather data from the Bureau of Meterology (BOM) then publishes said data locally via MQTT messages. BOM weather data is only for Australian locations. This weather data is obtained via an undocumented API, which was seemingly discovered, or at least popularised, thanks to [this Github repo](https://github.com/chris-horner/SocketWeather) - and [this one](https://github.com/bremor/bureau_of_meteorology). Because this is an undocumented API, it could be turned off at any time. BOM weather data is generally accepted as being the most accurate weather data in Australia.

The BOM weather data API is used in [this BOM weather site](https://weather.bom.gov.au/). Go to this website then enter your suburb / location in the search bar then click on the correct result. Take a note of the seven digit *geohash* in the URL in the address bar of your browser. This will be used to get data for your location and you need to add this to your config.toml file.

The main idea with this program is to collect the BOM weather data via the API, parse the data, check for null or erroneous entries, then publish the wanted data in a local network via MQTT messages. This way, we can have many programs do things with the MQTT data in a de-coupled manner. For example, we can setup alerts and notifications. We can use various home automations. We can also write the data to a database. By using an MQTT message bus, we can loosely couple all of these services, which provides desired reliability in our system. In essense, RusQTTbom becomes a local weather sensor providing BOM weather data.

==Note: This repo should be considered as being in the early *alpha* stage so is still in active initial development however it functionally works fine as is for testing or personal use==

The MQTT client library used is [rumqttc](https://github.com/bytebeamio/rumqtt). Weather data that is published by RusQTTbom via MQTT consists of the following values, by the used MQTT topics:

- outside/weather/current-temp
- outside/weather/temp-feels
- outside/weather/min-temp
- outside/weather/max-temp
- outside/weather/humidity
- outside/weather/rain-today
- outside/weather/wind-kms
- outside/weather/wind-kts
- outside/weather/wind-dir
- outside/weather/gusts
- outside/weather/max-gust

The RusQTTbom MQTT client name is 'rusqttbom'.

Configuration options are limited in this initial release. Options consist of the following:

- location name
- location geohash
- MQTT broker IP address
- MQTT broker port

More configuration options will be avaible in later releases. RusQTTbom expects the `config.toml` file to be saved under the users home directory in `/.config/rusqttbom/config.toml`. The program will not create the directory or move the config file there. Currently, a user needs to do this, however it only needs to be done once. This works MacOS and it *should* work on other OSs too. Let me know if there are issues.

Issues can be viewed [here](https://github.com/athenars-io/rusqttbom/issues), and the development plan can be found at the [Project](https://github.com/orgs/athenars-io/projects/1/views/2).

To use this program in its current state, the repo can be forked and cloned, configuration file edited and save in the appropriate file (as explained), then use `cargo build --release`. For best use of this program, be sure to set a CRON job to run the binary every 10mins. For the binary to run, it needs to be able to access the config.toml file in the expected directory as explained above. **Important!**: Do not hit this API more than once every ten minutes. Not abusing this API should help keep it from being shut down or modified.

Be sure to star this repo to watch the progress if this interests you!