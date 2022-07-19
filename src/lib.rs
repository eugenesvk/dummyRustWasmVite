#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
#![warn(unreachable_pub)]

use wasm_bindgen::prelude::	*;
use web_sys::              	console;

pub mod style;
pub mod components {
  pub mod logo;
  pub mod __tests__ {
    pub mod logo_test;
  }
}
mod composables {
  pub(crate) mod use_storage_local;
}

/// print to console.log, but only during debug builds: `[file:line]  log message`
#[macro_export] macro_rules! dbglog { ($($args:tt)*) => {
  #[cfg(    debug_assertions)] {
  $crate::log (std::format!("[{}:{}]  {}", std::file!(), std::line!(), std::format!($($args)*))); }
  #[cfg(not(debug_assertions))] { ($($args)*) } };}
/// print to console.log: `[file:line]  log message`
#[macro_export] macro_rules! log    { ($($args:tt)*) => {
  $crate::log (std::format!("[{}:{}]  {}", std::file!(), std::line!(), std::format!($($args)*))); };}
// #[macro_export] macro_rules! info { ($($args:tt)*) => {
  // $crate::info($crate::pretty_time()     , std::file!(), std::line!(), std::format!($($args)*));  };}
/// print to console.warn: `[file:line]  log message`
#[macro_export] macro_rules! warn   { ($($args:tt)*) => {
  $crate::warn(std::format!("[{}:{}]  {}", std::file!(), std::line!(), std::format!($($args)*))); };}
#[inline] pub fn log (s:String) {/*! console.log()  */ console::log_1  (&JsValue::from(s)); }
#[inline] pub fn warn(s:String) {/*! console.warn() */ console::warn_1 (&JsValue::from(s)); }
#[inline] pub fn err (s:String) {/*! console.error()*/ console::error_1(&JsValue::from(s)); }
