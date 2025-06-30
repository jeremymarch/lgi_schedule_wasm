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
    let start_date = "2025-06-09";
    //let juneteenth = "2025-06-19 08:30[America/New_York]".parse().unwrap();
    //let julyfour = "2025-07-04 08:30[America/New_York]".parse().unwrap();
    let holidays = vec!["2025-06-19", "2025-07-04"];
    let faculty = vec![
        vec!["BP", "JM", "HH", "EBH"],
        vec!["BP", "JM", "HH", "EBH"],
        vec!["BP", "JM", "HH", "EBH"],
        vec!["BP", "JM", "EBH"],
        vec!["BP", "JM", "EBH"],
        vec!["BP", "JM", "EBH"],
        vec!["ABF", "JM", "EBH"],
        vec!["ABF", "JM", "EBH"],
        vec!["ABF", "JM", "EBH"],
        vec!["ABF", "JM", "EBH"],
        vec!["ABF", "JM", "EBH"],
    ];
    let summer = create_summer(start_date, holidays, faculty).unwrap();

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

            let tr = document.create_element("tr").unwrap();
            table.append_child(&tr).unwrap();

            let td = document.create_element("td").unwrap();
            tr.append_child(&td).unwrap();
            td.set_inner_html(format!("Week {}", week_num).as_str());
            let _ = td.set_attribute("colspan", "9");

            let tr = document.create_element("tr").unwrap();
            table.append_child(&tr).unwrap();
            for col in 0..9 {
                let td = document.create_element("td").unwrap();
                tr.append_child(&td).unwrap();
                match col {
                    0 => td.set_inner_html("quiz"),
                    1 => td.set_inner_html(""),
                    2 => td.set_inner_html("8:30 a.m."),
                    3 => td.set_inner_html("9:30 a.m."),
                    4 => td.set_inner_html("10:40 a.m."),
                    5 => td.set_inner_html("12:15 p.m."),
                    6 => td.set_inner_html("1:00 p.m. ---------- "),
                    7 => td.set_inner_html(" ---------- 4:00 p.m."),
                    8 => td.set_inner_html("stats"),
                    _ => (),
                }
            }

            week_num += 1;
        }
        let tr = document.create_element("tr").unwrap();
        tr.set_class_name(get_weekday(row.date.weekday()).to_lowercase().as_str());
        table.append_child(&tr).unwrap();

        if row.exam.is_some() {
            for col in 0..6 {
                match col {
                    0 => {
                        let td = document.create_element("td").unwrap();
                        tr.append_child(&td).unwrap();
                    }
                    1 => {
                        let td = document.create_element("td").unwrap();

                        let day = format!(
                            "{}<br>{}{}",
                            get_weekday(row.date.weekday()),
                            row.date.strftime("%B %-d").to_string(),
                            if row.day < 1 {
                                String::from("")
                            } else {
                                format!("<br>Day {}", row.day)
                            }
                        );
                        td.set_inner_html(&day);
                        td.set_class_name("daycolumn");

                        tr.append_child(&td).unwrap();
                    }
                    2 => {
                        let td = document.create_element("td").unwrap();
                        let _ = td.set_attribute("colspan", "4");
                        td.set_inner_html("JM");
                        tr.append_child(&td).unwrap();
                    }
                    3 => {
                        let td = document.create_element("td").unwrap();
                        td.set_class_name("lecturecolumn");
                        let lecture_str = format!("{}<br>{}", row.lecture_title, row.lecture);
                        td.set_inner_html(&lecture_str);
                        tr.append_child(&td).unwrap();
                    }
                    4 => {
                        let td = document.create_element("td").unwrap();
                        let v = format!("Vocabulary Notes<br>{}", row.voc_notes.clone());
                        td.set_inner_html(&v);
                        tr.append_child(&td).unwrap();
                    }
                    5 => {
                        let td = document.create_element("td").unwrap();
                        tr.append_child(&td).unwrap();
                    }

                    _ => (),
                }
            }
        } else {
            for col in 0..9 {
                let td = document.create_element("td").unwrap();
                tr.append_child(&td).unwrap();
                match col {
                    0 => {
                        td.set_class_name("quizcolumn");
                        td.set_inner_html(row.quiz_grader.as_str());
                    }
                    1 => {
                        let day = format!(
                            "{}<br>{}{}",
                            get_weekday(row.date.weekday()),
                            row.date.strftime("%B %-d").to_string(),
                            if row.day < 1 {
                                String::from("")
                            } else {
                                format!("<br>Day {}", row.day)
                            }
                        );
                        td.set_inner_html(&day);
                        td.set_class_name("daycolumn");
                    }
                    2 => {
                        td.set_class_name("morningoptionalcolumn");
                        if let Some(s) = row.morning_optional.clone() {
                            let s = format!("(optional)<br>{}", s);
                            td.set_inner_html(&s);
                        }
                    }
                    3 => {
                        td.set_class_name("drill1column");
                        td.set_inner_html(get_drill_col(&row.drill1).as_str());
                    }
                    // quizzes
                    // find self-correcting
                    // 7-10 schedule
                    4 => {
                        td.set_class_name("drill2column");
                        td.set_inner_html(get_drill_col(&row.drill2).as_str());
                    }
                    5 => {
                        td.set_class_name("noonoptionalcolumn");
                        td.set_inner_html(&row.noon_optional1.clone());
                    }
                    6 => {
                        if row.friday_review1.len() > 0 {
                            td.set_class_name("fridayreviewcolumn1");
                            td.set_inner_html(
                                get_review_col(&row.friday_review1, row.day).as_str(),
                            );
                        } else {
                            td.set_class_name("lecturecolumn");
                            let lecture_str = format!("{}<br>{}", row.lecture_title, row.lecture);
                            td.set_inner_html(&lecture_str);
                        }
                    }
                    7 => {
                        if row.friday_review2.len() > 0 {
                            td.set_class_name("fridayreviewcolumn1");
                            td.set_inner_html(
                                get_review_col(&row.friday_review2, row.day).as_str(),
                            );
                        } else {
                            td.set_class_name("vocnotescolumn");
                            let v = format!("Vocabulary Notes<br>{}", row.voc_notes.clone());
                            td.set_inner_html(&v);
                        }
                    }
                    8 => td.set_class_name("statscolumn"),

                    _ => (),
                }
                if col == 2 && row.day < 1 {
                    let _ = td.set_attribute("colspan", "8");
                    break;
                }
            }
        }
    }
}

fn get_drill_col(fac: &Vec<String>) -> String {
    match fac.len() {
        2 => format!("E - {}<br>F - {}", fac[0], fac[1]),
        3 => format!("E - {}<br>F/G - {}<br>H - {}", fac[0], fac[1], fac[2]),
        _ => String::from(""),
    }
}

fn get_review_col(fac: &Vec<String>, day: u32) -> String {
    if day < 15 {
        format!("Review<br>E/F - {}<br>G/H - {}", fac[0], fac[1])
    } else {
        format!("Review<br>E - {}<br>F - {}", fac[0], fac[1])
    }
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
