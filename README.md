# RusQTTbom 

RusQTTbom collects weather data from the Bureau of Meterology (BOM) then publishes said data locally via MQTT messages. BOM weather data is only for Australian locations. This weather data is obtained via an undocumented API, which was discovered thanks to [this Github repo](https://github.com/bremor/bureau_of_meteorology). BOM weather data is generally accepted as being the most accurate weather data in Australia.

The main idea with this program is to collect the BOM weather data via the API, parse the data, then publish the wanted data in a local network via MQTT messages. This way, we can have many programs do things with the MQTT data in a de-coupled manner. For example, we can setup alerts and notifications. We can also setup and use various home automations. We can also write the data to a database. By using an MQTT message bus, we can loosely couple all of these services, which provides desired reliability.

==Note: This repo is new and this program has only just begun to be developed.==

This program aims to be a small highly reliable utility that enables subsequent use of BOM weather data in a data-centric smart home automation system. *Athenars* will be developing a range of these sorts of utilities that grab data from a range of places and publish MQTT messages on a local network.

Initially, limited configuration options will be available - consisting of a configuration file where users can input details like the desired weather data and MQTT broker details. Configuration and other user friendly options will be developed after the core functionality has been completed.

For now, this program does not work, though it is now in active development as the design for a range of utilities has now been completed. Once the initial build is complete, the repo will need to be cloned, configuration file edited, then a cargo build / run will need to suffice. Be sure to star this repo to watch the progress if this interests you!