#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]
#![warn(unreachable_pub)]
#![allow(non_upper_case_globals)]

use wasm_bindgen::prelude::                	*;
use crate::                                	log;
use crate::composables::use_storage_local::	{use_storage_local_fn, storage_local_api};
use wasm_bindgen_futures::                 	{JsFuture, spawn_local};

#[wasm_bindgen]
// pub async fn storage_logic() -> i32 {
pub fn storage_logic() -> i32 {
  let aaa = use_storage_local_fn();
  let bbb = storage_local_api();
  // let ddd = async { aaa.await + bbb.await };

  use futures::executor::block_on;
  // aaa.await;
  // block_on(aaa); // 'condvar wait not supported' panick, can't block in a browser (Async in the browser has to use the browser's executor, you can't bring your own, and you can't ever block on async functions)
  // spawn_local(async {aaa.await; ()}); // works on next tick
  spawn_local(async {aaa.await; ()}); // works on next tick
  3
}

/* storage.ts
import { useStorageLocal, storageLocalAPI }	from '~/composables/useStorageLocal'
import {createSharedRoot}                  	from '@solid-primitives/rootless'

const prefix 	= 'vitesse';
const key    	= 'webext_demo';
const prefkey	= [prefix,key].join('.');

const useState = createSharedRoot(() => {
  return useStorageLocal({api:storageLocalAPI, prefix:prefix, sync:true});
});

const [storageDemo, setStorageDemo, storageDemoAct] = useState()

if (typeof await storageDemo[key] === 'undefined') { await setStorageDemo(key, 'Storage Demo'); }
export { storageDemo, setStorageDemo, storageDemoAct, prefix, key, prefkey}
*/
