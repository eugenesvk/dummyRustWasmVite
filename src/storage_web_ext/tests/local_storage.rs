use wasm_bindgen::     	UnwrapThrowExt;
use serde_json::       	{Map, Value};
use wasm_bindgen_test::	{wasm_bindgen_test as test, wasm_bindgen_test_configure};

use storage_web_ext::	{LocalStorage, Storage};

// cargo test --target wasm32-unknown-unknown -p storage_web_ext
wasm_bindgen_test_configure!(run_in_browser);


#[test]
fn ggg() {
  assert_eq!(1, 1)
}

// // ERROR: browser is not defined, how to test in a web extension context?
// #[test]
// async fn get() {
//   let key  	= "key";
//   let value	= "value";
//   let _res 	= LocalStorage::set1(key, value).await;

//   let obtained_value:Map<String,Value> = LocalStorage::get(key)
//     .await
//     .expect_throw("unreachable: test get does not throw an exception");
//   let obtained_value:&Value = &obtained_value[key];
//   if let Value::String(s) = obtained_value {
//     let obtained_value:String = s.to_string();
//     assert_eq!(value, obtained_value);
//   }
// }

// // #[derive(Deserialize)]
// // struct Data {
// //   key1: String,
// //   key2: String,
// // }

// // #[test]
// // fn get_all() {
// //   LocalStorage::set("key1", "value").unwrap();
// //   LocalStorage::set("key2", "value").unwrap();

// //   let data: Data = LocalStorage::get_all().unwrap();
// //   assert_eq!(data.key1, "value");
// //   assert_eq!(data.key2, "value");
// // }

// // #[test]
// // fn set_and_length() {
// //   LocalStorage::clear();
// //   assert_eq!(LocalStorage::length(), 0);
// //   LocalStorage::set("key", "value").unwrap();
// //   assert_eq!(LocalStorage::length(), 1);
// //   LocalStorage::clear();
// //   assert_eq!(LocalStorage::length(), 0);
// // }









// // // Example of a test in a headless browser

// // #![cfg(test)]
// // use wasm_bindgen_test::	{wasm_bindgen_test as test, wasm_bindgen_test_configure};
// // // 1. test example fails with workspace default-members enabled, specify
// // // cargo test --target wasm32-unknown-unknown --lib rust_wasm_vite_dummy -p rust_wasm_vite_dummy
// // // #[test] fn pass1() { assert_eq!(1, 1);}

// // // 2. set webdrivers rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/browsers.html#chromedriverpathtochromedriver

// // // 3. configure to run tests in a headless browser
// // wasm_bindgen_test_configure!(run_in_browser);

// // // 4. set up the dominator's app to test

// // use std::sync::              	Arc;
// // use futures_signals::signal::	{Mutable};
// // use dominator::              	{Dom, html};
// // use rust_wasm_vite_dummy::   	{log, style::*, components::logo::logo};

// // const TEST_LOGO_ID  	: &str = "test_logo_id";
// // const TEST_LOGO_TEXT	: &str = "test_logo_text";

// // struct App { counter:Mutable<i32>, }

// // impl App {
// //   fn new() -> Arc<Self> {
// //     Arc::new(Self {
// //       counter:Mutable::new(0),
// //     })
// //   }

// //   fn render(state:Arc<Self>) -> Dom {
// //     html!("main", {
// //       .attr("id", TEST_LOGO_ID)
// //       .text(TEST_LOGO_TEXT)
// //       .children(&mut [
// //         logo(),
// //       ])
// //     })
// //   }
// // }

// // // 5a. render the test app ...

// // use wasm_bindgen::prelude::	*;
// // use wasm_bindgen::         	JsCast;
// // use web_sys::              	{console, HtmlElement};

// // #[wasm_bindgen]
// // pub async fn render_app() -> String {
// //   let window  	= web_sys::window()             	.expect("Global window does not exists");
// //   let document	= window.document()             	.expect("Expecting a document on window");
// //   let body    	= document.body()               	.expect("Document expect to have have a body");
// //   let elem    	= document.create_element("div")	.expect("Couldn't create a new div element");

// //   let app = App::new();
// //   dominator::append_dom(&elem, App::render(app));

// //   gloo_timers::future::TimeoutFuture::new(0).await;
// //   let elem_html	= elem.unchecked_into::<HtmlElement>();
// //   let html_text	= &elem_html.inner_text();
// //   html_text.to_string()
// // }

// // // 5a. ... and add a test assertion

// // #[test]
// // async fn test_app_output() {
// //   let html_text:String = render_app().await;
// //   assert_eq!(&html_text, &TEST_LOGO_TEXT);
// // }
