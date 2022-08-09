use wasm_bindgen::UnwrapThrowExt;

use crate::Storage;

/// Provides API to deal with `sessionStorage`
#[derive(Debug)]
pub struct SessionStorage;

impl Storage for SessionStorage {
  fn raw() -> web_extensions_sys::StorageArea {
    web_extensions_sys::browser
      .storage()
      .session()
  }
}
