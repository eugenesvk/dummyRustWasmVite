//! Various utility functions

use wasm_bindgen::	{JsValue as JsVal};
use js_sys::      	{Array as JsArr};

pub fn str_slice2js_array(strings: &[&str] ) -> JsArr { //! Convert a slice of str to a JS array of strings (useful in a `storage.local.get(keys)` where `keys` can be string|string[])
  let arr = JsArr::new_with_length(strings.len() as u32);
  for (i, s) in strings.iter().enumerate() {
    arr.set(i as u32, JsVal::from_str(s)); }
  arr
}
