## 2.11.1 - 17-05-2024

* Fix request time in proxy log to work with millisecond precision

## 2.11.0 - 27-02-2024

* Fix wasm integration by removing call to `SystemTime::now` which is not available in wasm
* Fix default local network for trusted proxies
* Add default header action

## 2.10.1 - 06-02-2024

* Fix building without specific features

## 2.10.0 - 06-02-2024

* Updated dependencies
* Fix request time in log
* Add backend duration to log

## 2.9.0 - 02-10-2023

* Updated dependencies
* Improve memory usage in router
* Improve performance when creating a router
* Improve performance when using html filters

## 2.8.0 - 26-05-2023

* Fix parsing bug in ip address for logs and request matching
* Fix incorrect behavior when matching a request against a response status code
* Add support for excluding methods in request matching
* Add support for excluding response status code in request matching

## 2.7.1 - 23-03-2023

* Add optional features to libredirectionio, to allow better compilation on wasm target

## 2.7.0 - 20-03-2023

* Add new trigger support for date, time and date time

## 2.6.0 - 08-03-2023

* Add support for deflate and brotli compression
* Avoid creating a body filter if there is only compression and no real filters
* Fix supporting host with port
* Add support to trusted proxies in log to avoid collecting their ips
* Support streaming body when there is compression

## 2.5.2 - 24-11-2022

* Fix a bug in wasm target compilation that make it impossible to compile

## 2.5.1 - 23-11-2022

* Ensure order for rules, no rules will always be applied in the same order even if they have the same priority, as they
  are sorted by their ID
* Body filters will now only be applied on a correct content-type header

## 2.5.0 - 28-10-2022

* Add support for gzip compression in body filter
* Hardened error handling in body filter

## 2.4.0 - 07-07-2022

* Update crate to publish on crates.io

## 2.3.0 - 13-04-2022

* Add support to trigger on specific ip address
* Add variable feature which allow using request properties in action values
* Add append/prepend/replace text content
* Fix cases when an url could not be parsed
* Fix wrong rule ids reported in log or header

## 2.2.0 - 21-04-2021

* Add cache support for cloudflare worker

## 2.1.1 - 16-03-2021

* Dependencies update

## 2.1.0 - 02-02-2021

* Allow to globally ignore and copy specific query parameter
* Add client ip to log
* Extract ip from forwraded header in log

## 2.0.0 - 11-01-2021

* Initial stable version of libredirectionio
