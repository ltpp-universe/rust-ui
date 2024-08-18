use wasm_bindgen::prelude::*;
mod init;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    init::init::run();
    Ok(())
}
