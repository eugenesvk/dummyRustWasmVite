#![warn(unreachable_pub)]

use wasm_bindgen::prelude::	*;
use rust_wasm_vite_dummy:: 	{log};

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
  log!("@background: log! message");
  Ok(())
}
