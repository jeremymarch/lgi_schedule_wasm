use jiff::civil::Weekday;
use lgi_schedule::*;
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

    create_table2();

    Ok(())
}
/*
#[wasm_bindgen]
pub fn create_table() {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let days = vec![
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];

    // Manufacture the element we're gonna append
    let table = document.create_element("table").unwrap();
    table.set_class_name("week-table week1");
    for row in 0..9 {
        let tr = document.create_element("tr").unwrap();
        table.append_child(&tr).unwrap();
        for col in 0..9 {
            let td = document.create_element("td").unwrap();
            tr.append_child(&td).unwrap();

            match row {
                0 => match col {
                    1 => {
                        td.set_inner_html("Week 1");
                        let _ = td.set_attribute("colspan", "9");
                        break;
                    }
                    _ => (),
                },
                1 => match col {
                    0 => td.set_inner_html("quiz"),
                    1 => td.set_inner_html(""),
                    2 => td.set_inner_html("8:30 a.m."),
                    3 => td.set_inner_html("9:30 a.m."),
                    4 => td.set_inner_html("10:40 a.m."),
                    5 => td.set_inner_html("12:15 a.m."),
                    6 => td.set_inner_html("1:00 p.m --------"),
                    7 => td.set_inner_html("---------- 4:00 p. m."),
                    8 => td.set_inner_html("stats"),
                    _ => (),
                },
                _ => {
                    if row > 1 && col == 1 {
                        td.set_inner_html(days[row - 2]);
                    }
                }
            }
        }
    }

    body.append_child(&table).unwrap();
}
*/

#[wasm_bindgen]
pub fn create_table2() {
    let start = "2025-06-09 08:30[America/New_York]";
    let faculty = vec![
        String::from("BP"),
        String::from("JM"),
        String::from("HH"),
        String::from("EBH"),
    ];
    let summer = create_summer(start, faculty).unwrap();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mut table = document.create_element("table").unwrap();

    let mut week_num = 1;
    for row in &summer.days_array {
        if row.date.weekday() == Weekday::Monday {
            table = document.create_element("table").unwrap();
            table.set_class_name(format!("week-table week{}", week_num).as_str());
            body.append_child(&table).unwrap();
            week_num += 1;
        }
        let tr = document.create_element("tr").unwrap();
        table.append_child(&tr).unwrap();
        for col in 0..9 {
            let td = document.create_element("td").unwrap();
            tr.append_child(&td).unwrap();
            match col {
                1 => td.set_inner_html(&get_weekday(row.date.weekday())),
                _ => (),
            }
        }
    }
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
