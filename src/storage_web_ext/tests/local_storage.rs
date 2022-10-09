#![cfg(test)]

use wasm_bindgen::     	UnwrapThrowExt;
use serde_json::       	{Map, Value};
use wasm_bindgen_test::	{wasm_bindgen_test as test, wasm_bindgen_test_configure};

use storage_web_ext::	{local_storage::LocalStorage, Storage};

wasm_bindgen_test_configure!(run_in_browser);
// cargo test --target wasm32-unknown-unknown -p storage_web_ext


#[test] fn local_storage_ggg_success() { assert_eq!(1, 1) }

// TODO: how to make these tests run in a web extension context?
use wasm_bindgen::prelude::*;
#[test]
fn open_options() {
	// ERROR: Cannot read properties of undefined (reading 'openOptionsPage')
  #[wasm_bindgen(inline_js="export function openOptionsPage() { chrome.runtime.openOptionsPage() }")]
  extern "C" { #[wasm_bindgen(js_name =     openOptionsPage)]
    pub fn open_options_page_inline();}
  open_options_page_inline();
}

// ERROR: browser is not defined, how to test in a web extension context?
// can't test in a headless mode, no chrome there, set $NO_HEADLESS=1 to disable
#[test] async fn get() {
  let key  	= "key";
  let value	= "value";
  let _res 	= LocalStorage::set1(key, value).await;

  let obtained_value:Map<String,Value> = LocalStorage::get(key)
    .await
    .expect_throw("unreachable: test get does not throw an exception");
  let obtained_value:&Value = &obtained_value[key];
  if let Value::String(s) = obtained_value {
    let obtained_value:String = s.to_string();
    assert_eq!(value, obtained_value);
  }
}

#[derive(Deserialize)] struct Data {key1:String,key2:String,}
#[test] async fn get_all() {
  LocalStorage::set1("key1","value1").await;
  LocalStorage::set1("key2","value2").await;
  let data: Data = LocalStorage::get_all().await;
  assert_eq!(data.key1, "value1");
  assert_eq!(data.key2, "value2");
}

#[test]
async fn set_and_length() {
  LocalStorage::clear();                  	assert_eq!(LocalStorage::length(), 0);
  LocalStorage::set1("key","value").await;	assert_eq!(LocalStorage::length(), 1);
  LocalStorage::clear();                  	assert_eq!(LocalStorage::length(), 0);
}
