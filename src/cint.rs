use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[link(name = "cint", kind = "static")]
extern "C" {
    #[wasm_bindgen]
    pub(crate) fn CINTlen_cart(l: i32) -> i32;
}
