# RusQTTbom 

RusQTTbom collects weather data from the Bureau of Meterology (BOM) then publishes said data locally via MQTT messages. BOM weather data is only for Australian locations. This weather data is obtained via an undocumented API, which was discovered thanks to [this Github repo](https://github.com/bremor/bureau_of_meteorology). BOM weather data is generally accepted as being the most accurate weather data in Australia.

The BOM weather data API is used in [this BOM weather site](https://weather.bom.gov.au/). Go to this website then enter your suburb / location in the search bar then click on the correct result. Take a note of the seven digit key in the URL in the address bar of your browser. This will be used to grab data for your location. Further instructions regarding configuration will be provided as development progresses.

The main idea with this program is to collect the BOM weather data via the API, parse the data, then publish the wanted data in a local network via MQTT messages. This way, we can have many programs do things with the MQTT data in a de-coupled manner. For example, we can setup alerts and notifications. We can use various home automations. We can also write the data to a database. By using an MQTT message bus, we can loosely couple all of these services, which provides desired reliability.

==Note: This repo should be considered as being in the *pre-alpha* stage so is still in active initial development==

Initially, limited configuration options will be available - consisting of a configuration file where users can input details like the desired weather data and MQTT broker details. Configuration and other user friendly options will be developed after the core functionality has been completed.

Issues can be viewed [here](https://github.com/athenars-io/rusqttbom/issues), and the development plan can be found at the [Project](https://github.com/orgs/athenars-io/projects/1/views/2).

Once the initial build is complete, the repo will need to be forked and cloned, configuration file edited, then a cargo build / run will need to suffice. Be sure to star this repo to watch the progress if this interests you!