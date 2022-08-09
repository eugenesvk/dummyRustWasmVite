#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, uncommon_codepoints))]
//! This crate provides wrappers for the [WebExtensions Storage API](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/storage)
//!
//! The data is stored in JSON form. We use [`serde`](https://serde.rs) for serialization and deserialization

//TODO replace wasm_bindgen with serde-wasm-bindgen for working with JsVals
//TODO above is Firefox APIs, might be different from Chrome's, though there is a working group working on standardizing/converging them somewhat
//TODO  #![deny(missing_docs, missing_debug_implementations)]
// use wasm_bindgen_futures::	{JsFuture as JsFut, spawn_local};
// use web_extensions_sys::  	{Local as ExtLocal};
  // @ wasm-bindgen-futures
  // fn spawn_local<F>(fut:F) where F:Future<Output=()> + 'static, {task::Task::spawn(Box::pin(fut));}
    // Runs a Rust `Future` on the current thread, which
    // must be `'static` because it will be scheduled to run in the background and cannot contain any stack references
    // will always be run on the next microtask tick even if it immediately returns `Poll::Ready`
    // Panic behavior as `future_to_promise`

use std::fmt::        	{Display, Debug};
use std::hash::       	{Hash, BuildHasher};
use std::collections::	HashMap;
use serde::           	{Deserialize, Serialize};
use serde_json::      	{Map, Value};
use wasm_bindgen::    	{prelude::*, JsValue as JsVal, UnwrapThrowExt}; // JsCast
use js_sys::          	{Array as JsArr, Object as JsObj};
use async_trait::     	async_trait;


use rust_wasm_vite_dummy::	{log, dbglog, util::{AsStrSlice, str_slice2js_array, GenericMap}};
use crate::errors::       	js_to_error;
use errors::              	StorageError;


pub mod errors;
pub mod local_storage;
pub mod session_storage;
pub use local_storage::  	LocalStorage;
pub use session_storage::	SessionStorage;

// pub mod logic {
//   pub mod storage;
// }


// #[path = "../tests"]
// pub mod tests {
//   pub mod local_storage;
// }


/// `storage_web_ext`'s `Result`
pub type Result<T> = std::result::Result<T    ,StorageError>;
/// Common `Result` type for web-extension-sys
pub type JsRes     = std::result::Result<JsVal,JsVal>;

#[async_trait(?Send)]
pub trait Storage { //! Trait which provides implementations for managing storage in the browser
  /// Get the raw [`web_extensions_sys::Local`] (Storage) instance
        fn raw         (      ) -> web_extensions_sys::StorageArea;

  // ToDO: get_with_def that receives a key-value object with default values
  async fn get    <K,R>(keys:K) -> Result<R > where R:for<'de>Deserialize<'de>,
  K:AsStrSlice + 'static, { //! Get the value(s) for the specified key(s)
    // TODO: if it's always returning a Map anyway, make R a concrete type instead of a generic?
    // key           string  or              (←↓spec)
    // keys array of strings or
    // keys object specifying default values (NOT implemented)
      // empty object/array   → an empty object             will be retrieved
      // null/undefined value → the entire storage contents will be retrieved (see `get_all`↓)
    let keys    :&[&str]   	= keys.as_slice();
    let keys_js :JsArr     	= str_slice2js_array(keys);
    let items_js           	= Self::raw()  // storage.local
      .get(&keys_js)       		// (this:&StorageAreaRead, keys:&JsVal) -> Future<JsRes>
      .await               		// JsRes ~ Ok(JsVal(Object({"field1":42})))
      .map_err(js_to_error)		// JsRes → Result<JsVal>
      .expect_throw(       		// JsVal ~ Object({"field1":42}))
        "unreachable: get_all does not throw an exception");
    let map_res	= items_js    // TODO this uses JSON.stringify, replace with another more efficient crate that skips the jsonification of the output
      .into_serde();
    let map:Map<String,Value>	= map_res?;

    dbglog!("@trait Storage @ fn get: keys={:?} | items_js={:?}", keys, items_js); //JsValue(Object({"field1":42}))
    dbglog!("@trait Storage @ fn get: map ={:?}", map); //={"field1":Number(42)}

    Ok(serde_json::from_value(Value::Object(map))?)
  }

  async fn get_all<R  >(      ) -> Result<R > where R:for<'de>Deserialize<'de> { //! Get all the stored keys and their values
    let keys               	= JsVal::NULL; // see `keys` spec at `get` ↑
    let items_js           	= Self::raw()  // storage.local
      .get(&keys)          		// (this:&StorageAreaRead, keys:&JsVal) -> Future<JsRes>
      .await               		// JsRes ~ Ok(JsVal(Object({"field1":42,...})))
      .map_err(js_to_error)		// JsRes → Result<JsVal>
      .expect_throw(       		// JsVal ~ Object({"field1":42,...}))
        "unreachable: get_all does not throw an exception");
    let map_res	= items_js    // TODO this uses JSON.stringify, replace with another more efficient crate that skips the jsonification of the output
      .into_serde();

    dbglog!("@trait Storage @ fn get_all: keys={:?} | items_js={:?}", keys, items_js); //JsValue(Object({"field1":42..}))
    dbglog!("@trait Storage @ fn get_all: map ={:?}", map_res); //=Ok({"field1":Number(42), "field2":String("blah"), "field3":Bool(true)})

    Ok(serde_json::from_value(Value::Object(map_res?))?)
  }

  async fn set  <M,K,V>(map:&M) -> Result<()> where M:GenericMap<K,V> + Serialize + ?Sized, { //! Insert a value(s) for the specified key(s) of the given key(s)/value(s) map
    // no `Map` trait, so can't accept both HashMap and BTreeMap https://stackoverflow.com/questions/54378172/rust-function-that-accepts-either-hashmap-and-btreemap
    // alt1: accept collections for keys/values?
    // alt2: add serde Map to GenericMap?
    // ToDo: a more efficient way to create JsObj? like str_slice2js_array?)
    let map_js = JsValue::from_serde(&map).unwrap_throw();
      // fn from_serde<T>(t:&T) -> Result<JsValue> where T:Serialize + ?Sized,
      // Creates a new JsValue from the JSON serialization of the object t provided. This function will serialize the provided value t to a JSON string, send the JSON string to JS, parse it into a JS object, and then return a handle to the JS object. This is unlikely to be super speedy so it’s not recommended for large payloads, but it’s a nice to have in some situations! Requires 'serde-serialize' feature
    dbglog!("@trait Storage @ fn set: map_js1={:?}", map_js); //{"setfield1": Number(24)}
    let map_js = JsObj::from(map_js);
      // JsObj::from_entries(iterable:&JsValue) -> Result<Object, JsValue>
      // JsObj::from        (obj     : JsValue) -> Object
      // JsObj::from        (obj     : Map    ) -> Object

    let items_js             	= Self::raw()  // storage.local
      .set(&map_js)          		// (this:&StorageAreaWrite, keys:&JsObj) -> Future<JsRes>
      .await                 		// JsRes ~ Ok(JsVal(undefined))
      .map_err(js_to_error)?;		// JsRes → Result<JsVal>
    dbglog!("@trait Storage @ fn set: items_js={:?}", items_js); //

    Ok(())
  }

  async fn set1   <K,V>(key:K,value:V) -> Result<()>
  where K:AsRef<str>, V:Serialize, { //! Insert a value for the specified key
    let key:&str	= key.as_ref();
    let mut hmap	= HashMap::new();
    hmap.insert(key, value);
    Self::set(&hmap).await
  }

  async fn delete <K  >(keys:K) where K:AsStrSlice + 'static, { //! Remove a key(s) and its(their) stored value(s)
    // key           string  or              (←↓spec)
    // keys array of strings
    let keys    :&[&str]	= keys.as_slice();
    let keys_js :JsArr  	= str_slice2js_array(keys);
    let items_js        	= Self::raw()  // storage.local
      .remove(&keys_js) 	// (this:&StorageAreaWrite, keys:&JsVal) -> Future<JsRes>
      .await            	// JsRes ~ Ok(JsValue(undefined))
      .expect_throw(    	// JsVal ~ undefined
        "unreachable: delete does not throw an exception");
  }

  async fn clear       (      )        { //! Remove all the stored data
    Self::raw()
      .clear()      	// (this:&StorageAreaWrite) -> Future<JsRes>
      .await        	// JsRes ~ Ok(JsValue(undefined))
      .expect_throw(	// JsVal ~ undefined
        "unreachable: clear does not throw an exception");
  }

  // ToDO: add this
  // getBytesInUse() doesn't exist in storage.local, but it's only a FireFox bug
  // async fn size   <K,R>(keys:K) -> Result<i32>,
  // K:AsStrSlice + 'static, { //! Get the bytes of storage space used by the item(s) matching the specified key(s)
    // key           string  or              (←↓spec)
    // keys array of strings or
      // empty      array   → 0                           will be retrieved
    // null/undefined value → the entire storage contents will be retrieved (see `size_all`↓)
  // }
}
