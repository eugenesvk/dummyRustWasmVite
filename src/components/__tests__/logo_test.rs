// Example of a test in a headless browser

#![cfg(test)]
use wasm_bindgen_test::	{wasm_bindgen_test as test, wasm_bindgen_test_configure};
// 1. test example fails with workspace default-members enabled, specify
// cargo test --target wasm32-unknown-unknown --lib rust_wasm_vite_dummy -p rust_wasm_vite_dummy
// #[test] fn pass1() { assert_eq!(1, 1);}
// tests must be in the root of the crate, or within a pub mod

// 2. set webdrivers rustwasm.github.io/docs/wasm-bindgen/wasm-bindgen-test/browsers.html#chromedriverpathtochromedriver

// 3. configure to run tests in a headless browser
wasm_bindgen_test_configure!(run_in_browser);

// 4. set up the dominator's app to test

use std::sync::              	Arc;
use futures_signals::signal::	{Mutable};
use dominator::              	{Dom, html};
use crate::                  	{log, style::*, components::logo::logo};

const TEST_LOGO_ID  	: &str = "test_logo_id";
const TEST_LOGO_TEXT	: &str = "test_logo_text";

struct App { counter:Mutable<i32>, }

impl App {
  fn new() -> Arc<Self> {
    Arc::new(Self {
      counter:Mutable::new(0),
    })
  }

  fn render(state:Arc<Self>) -> Dom {
    html!("main", {
      .attr("id", TEST_LOGO_ID)
      .text(TEST_LOGO_TEXT)
      .children(&mut [
        logo(),
      ])
    })
  }
}

// 5a. render the test app ...

use wasm_bindgen::prelude::	*;
use wasm_bindgen::         	JsCast;
use web_sys::              	{console, HtmlElement};

#[wasm_bindgen]
pub async fn render_app() -> String {
  let window  	= web_sys::window()             	.expect("Global window does not exists");
  let document	= window  .document()           	.expect("Expecting a document on window");
  let body    	= document.body()               	.expect("Document expect to have have a body");
  let elem    	= document.create_element("div")	.expect("Couldn't create a new div element");

  let app = App::new();
  dominator::append_dom(&elem, App::render(app));

  gloo_timers::future::TimeoutFuture::new(0).await;
  let elem_html	= elem.unchecked_into::<HtmlElement>();
  let html_text	= &elem_html.inner_text();
  html_text.to_string()
}

// 5a. ... and add a test assertion

#[test]
async fn test_app_output() {
  let html_text:String = render_app().await;
  assert_eq!(&html_text, &TEST_LOGO_TEXT);
}
