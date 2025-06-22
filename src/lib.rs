use wasm_bindgen::prelude::*;

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    create_table();

    Ok(())
}

#[wasm_bindgen]
pub fn create_table() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let table = document.create_element("table").unwrap();
    table.set_class_name("week-table week1");
    for row in 0..9 {
        let tr = document.create_element("tr").unwrap();
        table.append_child(&tr).unwrap();
        for col in 0..9 {
            let td = document.create_element("td").unwrap();
            tr.append_child(&td).unwrap();

            if row == 0 {
                td.set_inner_html("Week 1");
                let _ = td.set_attribute("colspan", "9");
                break;
            } else if row == 1 {
                match col {
                    0 => td.set_inner_html("quiz"),
                    1 => td.set_inner_html(""),
                    2 => td.set_inner_html("8:30 a.m."),
                    3 => td.set_inner_html("9:30 a.m."),
                    4 => td.set_inner_html("10:40 a.m."),
                    5 => td.set_inner_html("12:15 a.m."),
                    6 => td.set_inner_html("1:00 p.m --------"),
                    7 => td.set_inner_html("---------- 4:00 p. m."),
                    8 => td.set_inner_html(""),
                    _ => (),
                }
            }
        }
    }

    body.append_child(&table).unwrap();
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
