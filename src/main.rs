use wasm_bindgen::prelude::*;

pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let p = document.create_element("p")?;
    p.set_inner_html("Hello WASM!");
    let body = document.body().unwrap();
    body.append_child(&p)?;
    Ok(())
}
