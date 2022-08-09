// use wasm_bindgen::prelude::	*;
// use rust_wasm_vite_dummy:: 	{log};

// pub async fn app() -> Result<(), JsValue> {
//   log!("@popup: log! message");
//   Ok(())
// }

use std::sync::              	Arc;
use once_cell::sync::        	Lazy;
use futures_signals::signal::	{Mutable, SignalExt};
use dominator::              	{Dom, class, html, clone, events, svg};
use wasm_bindgen::           	{JsValue as JsVal};

use crate::               	open_options_page;
use rust_wasm_vite_dummy::	{style::*, components::logo::logo, logic::storage::storage_logic, util::str_slice2js_array};
use storage_web_ext::     	{LocalStorage, SessionStorage, Storage};


use wasm_bindgen_futures::	spawn_local;
use storage_web_ext::errors::StorageError;
use wasm_bindgen::UnwrapThrowExt;
use rust_wasm_vite_dummy::	{log, dbglog};
use serde_json::          	{Map, Value};
use std::collections::    	HashMap;

pub struct App { counter:Mutable<i32>, }

impl App {
  pub fn new() -> Arc<Self> {
    Arc::new(Self {
      counter:Mutable::new(0),
    })
  }

  pub fn render(state:Arc<Self>) -> Dom {
    static CLASS_ROOT          	: Lazy<[&str;5]> = Lazy::new(|| // Custom classes
      ["w-[300px]","px-4"      	,"py-5","text-center","text-gray-700"]);
    static CLA_CSS_ROOT        	: Lazy<String>   = Lazy::new(|| class! { // CSS styles in a Class
      .style("display"         	, "inline-block")
      .style("background-color"	, "black")
      .style("padding"         	, "10px") });
    static CLA_CSS_TEXT        	: Lazy<String>   = Lazy::new(|| class! {
      .style("color"           	, "white")
      .style("font-weight"     	, "bold") });
    static CLA_CSS_BUTTON      	: Lazy<String>   = Lazy::new(|| class! {
      .style("display"         	, "block")
      .style("width"           	, "100px")
      .style("margin"          	, "5px") });
    static CLA_CSS_BUTTON_HOVER	: Lazy<String>   = Lazy::new(|| class! {
      .style("background-color"	, "red")
      .style("margin"          	, "5px") });

    // remove test
      struct GetDefVals<'a> { field1:&'a str}
      let ccc = storage_logic();

    let set = spawn_local(async {
      let key  	= "loc_keys";
      let value	= "loc_value";
      let res  	= LocalStorage::set1(key, value).await;
      dbglog!("@popup @ render: set1_loc   ={:?}", res   ); //

      let obtained_value:Map<String,Value> = LocalStorage::get(key)
        .await
        .expect_throw("unreachable: test does not throw an exception");
      dbglog!("@popup @ render: obtained_value   ={:?}", obtained_value   ); //
      let obtained_value:&Value = &obtained_value[key];
      if let Value::String(n) = obtained_value {
        let obtained_value:String = n.to_string();
        dbglog!("@popup @ render: obtained_value   ={:?}", obtained_value   ); //
        assert_eq!(value, obtained_value);
      }

      let s_key  	= "sess_keys";
      let s_value	= "sess_value";
      let s_res  	= SessionStorage::set1(s_key, s_value).await;
      dbglog!("@popup @ render: set1_ses   ={:?}", s_res   ); //

      let obtained_ses_value:Map<String,Value> = SessionStorage::get(s_key)
        .await
        .expect_throw("unreachable: test does not throw an exception");
      dbglog!("@popup @ render: obtained_value   ={:?}", obtained_ses_value   ); //
      let obtained_ses_value:&Value = &obtained_ses_value[s_key];
      if let Value::String(n) = obtained_ses_value {
        let obtained_ses_value:String = n.to_string();
        dbglog!("@popup @ render: obtained_value   ={:?}", obtained_ses_value   ); //
        assert_eq!(s_value, obtained_ses_value);
      }

      ()
    });
    //
    html!("main", { // Create the DOM nodes
      // .attr("id", "app") // it's already mounted on an element with id("app")
      .class(*CLASS_ROOT) // .class(&*CLA_CSS_ROOT)

      .children(&mut [
        logo(),

        html!("div"   , {.text("Popup")}),
        html!("p"     , {.text("This is the popup page")
          .class(["mt-2","opacity-50"]) }),
        html!("button", {.text("Open Options")
          .apply(mixin_btn)
          .event(clone!(state => move |_:events::Click| {
            open_options_page();
          }))
        }),
        html!("div", {
          .class(["mt-2"])
          .child(html!("span", {
              .text("Storage: ")
              .text(&ccc.to_string())
              .class(["opacity-50"])
            } ))
    // TODO: replace with key_value() from storage
          .text_signal(state.counter.signal().map(|x| format!("divclas{}", x)))
        }),


    // TODO: remove the extra buttons
        html!("button", {
          .attr("id", "aaa")
          .class(["btn","mt-2"])
          .apply(mixin_btn)
          .class(&*CLA_CSS_BUTTON)
          .text("Increase")
          .event(clone!(state => move |_:events::Click| {
            state.counter.replace_with(|x| *x + 1); // Increment the counter
          }))
        }),

        html!("button", {
          .attr("id", "bbb")
          .class(&*CLA_CSS_BUTTON)
          .apply(mixin_icon_btn)
          // .apply(class_on_hover(&*CLA_CSS_BUTTON_HOVER))
          // .apply(class_on_hover(["xx-500","yy-400"]))
          .text("Decrease")
          .event(clone!(state => move |_:events::Click| {
            state.counter.replace_with(|x| *x - 1); // Decrement the counter
          }))
        }),

        html!("button", {
          .attr("id", "ccc")
          // .class(&*CLA_CSS_BUTTON)
          .apply(mixin_pseudo_style)
          .text("Reset")
          .event(clone!(state => move |_:events::Click| {
            state.counter.set_neq(0); // Reset the counter to 0
          }))
        }),

        html!("div", {
          .class(&*CLA_CSS_TEXT)
          .text_signal(state.counter.signal().map(|x| format!("Counter: {}", x)))
        }),


      ])
    })
  }
}
