#![macro_use]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

mod fib;

fn macro_console_log(txt: String) {
    console_log!("Hello {}!", txt);
}

#[wasm_bindgen]
pub struct BindgenExamples {
    int_value: u32,
}

#[wasm_bindgen]
impl BindgenExamples {
    pub fn console_log(&self, txt: String) {
        macro_console_log(txt);
    }

    pub fn print_my_int_value(&self) {
        macro_console_log(self.int_value.to_string());
    }

    pub fn alert(&self, txt: &str) {
        alert(&format!("Hello, {}!", txt));
    }

    pub fn create_hello_dom_element(&self) -> Result<(), JsValue> {
        let window = web_sys::window().expect("global `window` exists");
        let document = window.document().expect("should have a document on window");

        let val = document.create_element("p")?;
        val.set_inner_html("Wasm says hello!");

        document
            .get_element_by_id("wasm-area")
            .expect("document should have #wasm-dom on DOM")
            .dyn_ref::<HtmlElement>()
            .expect("#wasm-dom should be an HtmlElement")
            .append_child(&val)
            .expect("Hello element should have been added");

        Ok(())
    }

    pub fn time_fibonacci(&self, n: u32) {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");

        let t0 = performance.now();
        let res = fib::fibonacci(n);
        let t1 = performance.now();
        let duration = t1 - t0;

        fib::display_fibonacci_result(duration, res).unwrap();
    }

    pub fn new(val: u32) -> BindgenExamples {
        let int_value = val;
        BindgenExamples { int_value }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);

    fn alert(s: &str);
}
