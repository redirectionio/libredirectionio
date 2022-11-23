## 2.5.1 - 23-11-2022

 * Ensure order for rules, no rules will always be applied in the same order even if they have the same priority, as they are sorted by their ID
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
