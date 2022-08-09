use crate::EventTarget;
use wasm_bindgen::prelude::*;

// TODO
#[wasm_bindgen]
extern "C" {
    pub type BrowserAction;

    #[wasm_bindgen(method, getter, js_name = onClicked)]
    pub fn on_clicked(this: &BrowserAction) -> EventTarget;
}
