#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
#![warn(unreachable_pub)]
#![allow(non_upper_case_globals)]

use wasm_bindgen::prelude::	*;
use crate::popup::         	App;
use rust_wasm_vite_dummy:: 	{log, style::init_stylesheet};

pub mod popup;


// 1a bind WebExtension API via crates.io/crates/web-extensions-sys
use web_extensions_sys::browser;
pub fn open_options_page() {
  browser.runtime().open_options_page();
}

// 1b bind WebExtension API (use browser.runtime().open_options_page_nat();)
// #[wasm_bindgen] extern "C" {
//   pub type Browser;

//   pub static browser	: Browser; // used for Chrome Extensions with github.com/mozilla/webextension-polyfill or Firefox Addons
//   pub static chrome 	: Browser; // used for Chrome Extensions

//   #[wasm_bindgen	(method, getter)]
//   pub fn runtime	(this: &Browser) -> Runtime;
// }
// #[wasm_bindgen] extern "C" {
//   pub type Runtime;

//   #[wasm_bindgen(method, js_name = openOptionsPage)]
//   pub fn open_options_page_nat(this:&Runtime);
// }

// 1c use inline JS snippet
#[wasm_bindgen(inline_js="export function openOptionsPage() { chrome.runtime.openOptionsPage() }")]
extern "C" { #[wasm_bindgen(js_name =     openOptionsPage)]
	pub fn open_options_page_inline();}



#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
  #[cfg(debug_assertions)]
  console_error_panic_hook::set_once();

  init_stylesheet();
  //TODO remember the old counter instead of recreating the new one on every click? or not?
  let app = App::new();

  // 1a. append the dom via the default method OR
  // dominator::append_dom(&dominator::body(), App::render(app));

  // 1b. alternative: mount on an existing div id 'app' OR
  let app_elem = dominator::get_id("app");    // 1b1. get "app" element via dominator

  // 1b2. get "app" element manually
  use web_sys::{console, HtmlElement, Node};
  let window  	= web_sys::window()                	.expect("Global window does not exists");
  let document	= window.document()                	.expect("Expecting a document on window");
  let body    	= document.body()                  	.expect("Document expect to have have a body");
  let elem    	= document.get_element_by_id("app")	.expect("Element with id `app` not present");

  // 1b3. replace inner text
  use wasm_bindgen::JsCast;
  let elem_html	= elem.unchecked_into::<HtmlElement>();
  let html_text	= &elem_html.inner_text();
  let app_node 	= Node::from(elem_html);
  app_node.set_text_content(Some(""));
  log!("Old inner text: {}", &html_text);

  dominator::append_dom(&app_elem, App::render(app)); // or dominator::append_dom(&app_node...


  Ok(())
}
