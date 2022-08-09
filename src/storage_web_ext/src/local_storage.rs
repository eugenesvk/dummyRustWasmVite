use wasm_bindgen::UnwrapThrowExt;

use crate::Storage;

/// Provides API to deal with `storage.local`
#[derive(Debug)]
pub struct LocalStorage;

impl Storage for LocalStorage {
  fn raw() -> web_extensions_sys::StorageArea {
    web_extensions_sys::browser
      .storage()
      .local()
  }
}
