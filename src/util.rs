//! Various utility functions

use wasm_bindgen::	{JsValue as JsVal};
use js_sys::      	{Array as JsArr};

pub fn str_slice2js_array(strings: &[&str] ) -> JsArr { //! Convert a slice of str to a JS array of strings (useful in a `storage.local.get(keys)` where `keys` can be string|string[])
  let arr = JsArr::new_with_length(strings.len() as u32);
  for (i, s) in strings.iter().enumerate() {
    arr.set(i as u32, JsVal::from_str(s)); }
  arr
}


use std::slice::from_ref;
/// Implement a trait to convert &str or a collection thereof into a slice of &str
///```
///fn print_as_slice<T>(slice:T) where T:AsStrSlice {
///  let slice = slice.as_slice(); println!(".as_slice = {:?}", slice); }
///print_as_slice(     "1string literal");
///print_as_slice(    ["2str_lit@Arr#1","2str_lit@Arr#2"]);
///print_as_slice(vec!["3str_lit@Vec#1","3str_lit@Vec#2"]);
///```
// users.rust-lang.org/t/make-a-function-accept-either-str-or-str-as-an-argument/79139/9
pub trait          	AsStrSlice                	{fn as_slice(&self) -> &[&str];                }
impl               	AsStrSlice for     &str   	{fn as_slice(&self) -> &[&str] {from_ref(self)}}
impl               	AsStrSlice for   &[&str]  	{fn as_slice(&self) -> &[&str] {         self }}
impl<const N:usize>	AsStrSlice for    [&str;N]	{fn as_slice(&self) -> &[&str] {         self }}
impl               	AsStrSlice for Vec<&str>  	{fn as_slice(&self) -> &[&str] {         self }}

