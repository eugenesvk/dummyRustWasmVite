use crate::Storage;

/// Provides API to deal with `storage.session`
#[derive(Debug)]
pub struct       SessionStorage;

impl Storage for SessionStorage {
  fn raw() -> web_extensions_sys::StorageArea {
    web_extensions_sys::browser
      .storage()
      .session()
  }
}
