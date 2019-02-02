use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

pub fn display_fibonacci_result(duration: f64, value: u32) -> Result<(), JsValue> {
    let window = web_sys::window().expect("global `window` exists");
    let document = window.document().expect("should have a document on window");

    let res_html_element = document.create_element("p")?;
    res_html_element.set_inner_html(&format!(
        "WASM duration: {} ms, result: {}",
        duration, value
    ));

    document
        .get_element_by_id("fibonacci-time")
        .expect("document should have #fibonacci-time on DOM")
        .dyn_ref::<HtmlElement>()
        .expect("#fibonacci-time should be an HtmlElement")
        .append_child(&res_html_element)
        .expect("duration and value should have been added");

    Ok(())
}
