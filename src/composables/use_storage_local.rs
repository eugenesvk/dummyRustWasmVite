#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
// #![warn(unreachable_pub)]
#![allow(unreachable_pub)]
#![allow(non_upper_case_globals)]
#![allow(unused_must_use)]

use crate::                	log;
use wasm_bindgen::prelude::	*;
use wasm_bindgen_futures:: 	{JsFuture, spawn_local};

// use web_extensions_sys::	{StorageAreaWrite, Sync, Local, Managed, Storage, StorageChange};
use web_extensions_sys::   	{browser, Storage, StorageArea};
use js_sys::               	{Object as JsObj, Map as JsMap, Promise as JsPromise};


pub fn my_local_storage() -> StorageArea {
  browser.storage().local()
}


// replace mock API with real storage API when it's added to crates.io/crates/web-extensions

use futures::future::Future;

// I
use core::pin::Pin;
use futures::future::BoxFuture;
pub enum Awaitable<'a, T> {
  F(BoxFuture<'a, T>),
  C(T) }
pub enum StringOrNull{
  S(String),
  N(()) }

pub struct Test<'a> {
  item1:Awaitable<'a, StringOrNull>,
  item2:Awaitable<'a, ()          >,
  item3:Awaitable<'a, ()          >,
}

pub struct AsyncStorageActions { }

use dominator::	Dom;
use std::sync::	Arc;
impl AsyncStorageActions {
  // pub fn curr() -> Cur<String> {
    // Cur(String::new())
  // }
  // pub fn futt() -> Fut<()> {
    // Fut(async {()})
  // }
  pub fn render(state:Arc<Self>) -> () {}

  // pub fn aw() -> Awaitable<'static, String> {
    // Awaitable::C::<String>(C {String::new()})
    // Awaitable::C::<String>(C {"a".to_string()})
  // }

  // pub fn remove	() { |key:String|	Option<T>;} //Promise<void> | void;
  // pub fn clear 	() { ||          	();} //Promise<void> | void;
  // pub fn error 	() { ||          	Err();} //Error | undefined;
  // pub fn toJSON	() { ||          	();} //Promise<{ [key:string]:T;}>;
}
/*
export type Awaitable<T> = Promise<T> | T
export interface StorageLikeAsync {
  getItem   	(key:string)              	: Awaitable<string | null>
  setItem   	(key:string, value:string)	: Awaitable<void>
  removeItem	(key:string)              	: Awaitable<void>
}
*/


// II
pub enum Awaitable2<T> { F(Fut2), C(Cur2<T>) }
pub struct Fut2(JsPromise);
pub struct Cur2<T>(T);


async fn get_from_js() -> Result<JsValue, JsValue> {
  let promise = JsPromise::resolve(&42.into());
  let result  = JsFuture::from(promise).await?;
  Ok(result)
}

async fn tst() -> () {
  use futures::future::  	{ok, Ready, ready, lazy, Lazy, TryFutureExt};
  use futures::executor::	block_on;
  // or use ok::<_,()> if a compiler can't infer error type
  let future:Ready<Result<String,()>> = ok(String::from("hello")); //Create a future that is immediately ready with a success value
  let fut_r:Ready<        String> = ready(String::from("hello")); //Creates a future that is immediately ready with a value
  let fut_l = lazy(|_| 1); // Creates a new future that allows delayed execution of a closure. The provided closure is only run once the future is polled
  // let fut_l:Lazy<dyn FnOnce(&mut Context<'_>) -> i32> = lazy(|_| 1); // guess can't define a type for a closure

  // pub struct Ready<T>(_); Future for the ready function
  // async { assert_eq!(Ok(String::from("hello")), future.await);}; // can't use async in a sync
  let fut_block = block_on(future);
  assert_eq!(Ok(String::from("hello")), fut_block);
}


pub trait Storage1 { // Trait which provides implementations for managing storage in the browser.
  fn get<T,K>(key:K) // Get the value for the specified key
    where
      T: for<'de> Deserialize<'de>,
      K: AsRef<str>,
      {
    // let key  = key.as_ref();
    // let item = Self::raw()
    //   .get_item(key)
    //   .expect_throw("unreachable: get_item does not throw an exception")
    //   .ok_or_else(|| StorageError::KeyNotFound(key.to_string()))?;
    // let item = serde_json::from_str(&item)?;
    // Ok(item)
    ()
  }
}




pub(crate) struct Store {
  local_storage	: web_extensions_sys::Storage,
  // data      	: ItemList,
  data         	: JsObj,
  name         	: String,
}


use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// serde-wasm-bindgen not strictly compatible with either serde_json or, correspondingly, JsValue::from_serde / JsValue::into_serde, by default, for better compatibility with common JavaScript idioms and representations
// If you need compatibility with them, or you want to use JSON.stringify on the result without data loss, use Serializer::json_compatible() as serializer
// #[derive(Serialize, Deserialize)]
// pub struct Example {
//     pub field1: HashMap<u32, String>,
//     pub field2: Vec<Vec<f32>>,
//     pub field3: [f32; 4],
// }
// #[wasm_bindgen]
// pub fn send_example_to_js() -> JsValue {
//     let mut field1 = HashMap::new();
//     field1.insert(0, String::from("ex"));
//     let example = Example {
//         field1,
//         field2: vec![vec![1., 2.], vec![3., 4.]],
//         field3: [1., 2., 3., 4.]
//     };
//     JsValue::from_serde(&example).unwrap_throw() // Unwrap this `Option`/`Result`, but instead of panicking on failure throw an exception to JavaScript
// }



// Result<(), JsValue> change later
use futures::future::OptionFuture;
pub(crate) async fn use_storage_local_fn()  -> JsValue {
  let mut a: OptionFuture<_> = Some(async { 123 }).into();
  assert_eq!(a.await, Some(123));
  a = None.into();
  let a_await = a.await;
  assert_eq!(a_await, None);



  // let promise	= js_sys::Promise::resolve(&JsValue::from(42));	// Create a promise that is ready on the next tick of the micro task queue.
  // let x      	= JsFuture::from(promise).await.unwrap();      	// Convert that promise into a future and make the test wait on it.

  let some_supported_rust_value	= ("Hello, world!", 42);
  let js_value:JsValue         	= serde_wasm_bindgen::to_value(&some_supported_rust_value).unwrap();

  // use serde_wasm_bindgen::{Serializer, Deserializer};
  // use serde::{Serialize,Deserialize};
  // #[derive(Serialize, Deserialize)]
  // pub struct SomeStruct {
  //   pub name: str,
  //   pub data: i32,
  // }

  // 1 works, but serde_wasm_bindgen gives JSvalue, so need to convert to JsObj
  #[derive(Serialize)]
  struct Regular<'a> {
    field1	: &'a f32,
    field2	: &'a String,
    field3	: &'a bool
  }
  let typed_rust_structure = Regular {
    field1	: &42.0,
    field2	: &"blah".to_string(),
    field3	: &true,};
  let js_plain_val    = serde_wasm_bindgen::to_value(&typed_rust_structure).unwrap();
  let js_obj4c:JsObj   = js_plain_val.into(); // ::try_from(&).unwrap(); ::from();

  let js1 = JsValue::null();
  use web_extensions_sys::browser;
  let fut1 = my_local_storage(); //.get(&js1);
  // let fut2 = fut1.set(&obj);
  // let fut3 = JsFuture::from(fut2);
  // let fut = JsFuture::from(storage_local().get(&JsValue::null()));
  // spawn_local(async  move {
    // let db = fut2.await?;
  // });

  use wasm_bindgen::                 	JsCast;
  let locst:StorageArea              	= my_local_storage();
  let tt2                            	= locst.set(&js_obj4c);
  let set_res:Result<JsValue,JsValue>	= tt2.await;
  let f = match set_res {
    Ok (ok ) => ok,
    // Err(err) => panic!("Problem setting local storage: {:?}", err),
    Err(err) => {log!("Problem setting local storage: {:?}", err); JsValue::null()},
  };
  // spawn_local(ttt);

  let aaa = 1;
  f

  // 1a bind WebExtension API via crates.io/crates/web-extensions-sys
  // use web_extensions_sys::storage::{StorageAreaRead, StorageAreaWrite};
  // use web_extensions_sys::{StorageAreaRead, StorageAreaWrite};
  // pub use crate::storage::{StorageAreaRead, StorageAreaWrite};

  // use web_extensions_sys::browser as browser_ext;
  // fn storage_set() {
  //   browser_ext.runtime().open_options_page();
  // }

  // type StorageAreaRead;
  // #[wasm_bindgen(catch, method, js_name = "getBytesInUse")]
  // async fn get_bytes_in_use(this: &StorageAreaRead, keys: &JsValue) -> Result<JsValue, JsValue>;
  // async fn get(this: &StorageAreaRead, keys: &JsValue) -> Result<JsValue, JsValue>;

  // type StorageAreaWrite; #[wasm_bindgen(extends = StorageAreaRead)]
  // async fn set   	(this:&StorageAreaWrite, keys: &Object) 	-> Result<JsValue, JsValue>;
  // async fn remove	(this:&StorageAreaWrite, keys: &JsValue)	-> Result<JsValue, JsValue>;
  // async fn clear 	(this:&StorageAreaWrite)                	-> Result<JsValue, JsValue>;
}

pub(crate) async fn storage_local_api() -> i32 {
  let bbb = 3;
  bbb
}


// #[wasm_bindgen] extern "C"            {fn alert(s:&str);} // import a function alert from JS
// #[wasm_bindgen] pub fn greet(nm:&str) {   alert(&format!("Hello, {}!", nm));} // Export a `greet` function from Rust to JavaScript, that alerts a hello message

/*
from tab org: but window are WEB apis, not webext apis (browser.storage().local() is webext)
use web_sys::{window, Window, Performance, Storage, Blob, Url, BlobPropertyBag, FileReader};
thread_local! { static STORAGE: Storage = WINDOW.with(|w| w.local_storage().unwrap().unwrap());}
fn loc_store_get(key:&str) -> Option<String>	{STORAGE.with(|x| x.get_item(key      ).unwrap())}
fn loc_store_set(key:&str, value: &str)     	{STORAGE.with(|x| x.set_item(key,value).unwrap())}

impl Database {
  pub fn new() -> impl Future<Output = Result<Self, JsValue>> {
    let fut = JsFuture::from(browser.storage().local().get(&JsValue::null())); // TODO move this inside the async ?
    async move {
      let db = fut.await?;
      let db: Object = db.unchecked_into();
      Ok(Self::new_from_object_(db, true))
    }
  }
*/
