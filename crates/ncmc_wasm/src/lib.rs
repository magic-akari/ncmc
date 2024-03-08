use js_sys::Uint8Array;
use ncm_core::decoder::Decoder;
use ncm_meta::Encoder;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn convert(input: &[u8]) -> Result<Uint8Array, String> {
    let reader = std::io::Cursor::new(input);
    let decoder = Decoder::decode(reader).map_err(|e| e.to_string())?;
    let Encoder { data, .. } = Encoder::encode(decoder).map_err(|e| e.to_string())?;
    Ok((&*data).into())
}
