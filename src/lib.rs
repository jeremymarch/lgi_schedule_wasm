use jiff::civil::Weekday;
use lgi_schedule::*;
use wasm_bindgen::prelude::*;

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    // let window = web_sys::window().expect("no global `window` exists");
    // let document = window.document().expect("should have a document on window");
    // let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    // let val = document.create_element("p")?;
    // val.set_inner_html("Hello from Rust!");
    // body.append_child(&val)?;

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
    //let juneteenth = "2025-06-19 08:30[America/New_York]".parse().unwrap();
    //let julyfour = "2025-07-04 08:30[America/New_York]".parse().unwrap();

    let start_date = "2025-06-09";
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

    let lectures = vec![
        "EBH", "JM", "HH", "EBH", "HH", "JM", "BP", "HH", "JM", "HH", "BP", "EBH", "JM", "EBH",
        "JM", "BP", "EBH", "BP", "JM", "EBH", "BP", "EBH", "JM",
    ];

    let p = Params {
        faculty,
        start_date,
        holidays,
        lecture_assignments: lectures,
    };

    let summer = create_summer(&p).unwrap();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mut table = document.create_element("table").unwrap();

    let mut week_num = 1;
    for row in &summer.days_array {
        if row.date.weekday() == Weekday::Monday {
            table = document.create_element("table").unwrap();
            table.set_class_name(format!("week-table week{week_num}").as_str());
            body.append_child(&table).unwrap();

            let tr = document.create_element("tr").unwrap();
            table.append_child(&tr).unwrap();

            let td = document.create_element("td").unwrap();
            tr.append_child(&td).unwrap();
            td.set_inner_html(format!("Week {week_num}").as_str());
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

        if row.day == 1 {
            for col in 0..8 {
                let td = document.create_element("td").unwrap();
                tr.append_child(&td).unwrap();
                match col {
                    0 => (),
                    1 => {
                        let day = format!(
                            "{}<br>{}{}",
                            get_weekday(row.date.weekday()),
                            row.date.strftime("%B %-d"),
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
                        //td.set_class_name("morningoptionalcolumn");

                        td.set_inner_html("Orientation");
                    }
                    3 => {
                        //td.set_class_name("morningoptionalcolumn");
                        if let Some(s) = row.day_one_lectures.clone() {
                            let s = format!("Grammar<br>{}", s[0]);
                            td.set_inner_html(&s);
                        }
                    }
                    4 => {
                        //td.set_class_name("morningoptionalcolumn");
                        if let Some(s) = row.day_one_lectures.clone() {
                            let s = format!("Alphabet<br>{}", s[1]);
                            td.set_inner_html(&s);
                        }
                    }
                    5 => {
                        //td.set_class_name("morningoptionalcolumn");

                        td.set_inner_html("Lunch");
                    }
                    6 => {
                        //td.set_class_name("morningoptionalcolumn");
                        if let Some(s) = row.day_one_lectures.clone() {
                            let s = format!("Lecture on Accents<br>{}", s[2]);
                            td.set_inner_html(&s);
                            let _ = td.set_attribute("colspan", "2");
                        }
                    }
                    7 => {
                        td.set_class_name("statscolumn");
                        let v = row.get_stats();
                        let o = format!("{v:?}");
                        td.set_inner_html(&o);
                    }
                    _ => (),
                }
            }
        } else if row.exam.is_some() {
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
                            row.date.strftime("%B %-d"),
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
                        if let Some(lecture_title) = row.lecture_title.as_ref()
                            && let Some(lecture) = row.lecture.as_ref()
                        {
                            let lecture_str = format!("{lecture_title}<br>{lecture}");
                            td.set_inner_html(&lecture_str);
                        }
                        tr.append_child(&td).unwrap();
                    }
                    4 => {
                        let td = document.create_element("td").unwrap();
                        if let Some(voc) = row.voc_notes.as_ref() {
                            let v = format!("Vocabulary Notes<br>{voc}");
                            td.set_inner_html(&v);
                        }
                        tr.append_child(&td).unwrap();
                    }
                    5 => {
                        let td = document.create_element("td").unwrap();

                        td.set_class_name("statscolumn");
                        let v = row.get_stats();
                        let o = format!("{v:?}");
                        td.set_inner_html(&o);

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
                        if let Some(quiz) = row.quiz_grader.as_ref() {
                            td.set_inner_html(quiz);
                        }
                    }
                    1 => {
                        let day = format!(
                            "{}<br>{}{}",
                            get_weekday(row.date.weekday()),
                            row.date.strftime("%B %-d"),
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
                        if let Some(s) = row.morning_optional.as_ref() {
                            let s = format!("(optional)<br>{s}");
                            td.set_inner_html(&s);
                        }
                    }
                    3 => {
                        td.set_class_name("drill1column");
                        td.set_inner_html(get_drill_col(&row.get_drill1()).as_str());
                    }
                    // quizzes
                    // find self-correcting
                    // 7-10 schedule
                    4 => {
                        td.set_class_name("drill2column");
                        td.set_inner_html(get_drill_col(&row.get_drill2()).as_str());
                    }
                    5 => {
                        td.set_class_name("noonoptionalcolumn");
                        td.set_inner_html(&get_noon_optional_col(
                            &row.noon_optional1_title,
                            &row.noon_optional1,
                            &row.noon_optional2_title,
                            &row.noon_optional2,
                        ));
                    }
                    6 => {
                        if !row.friday_review1.is_empty() {
                            td.set_class_name("fridayreviewcolumn1");
                            td.set_inner_html(
                                get_review_col(&row.friday_review1, row.day).as_str(),
                            );
                        } else {
                            td.set_class_name("lecturecolumn");
                            if let Some(lecture_title) = row.lecture_title.as_ref()
                                && let Some(lecture) = row.lecture.as_ref()
                            {
                                let lecture_str = format!("{lecture_title}<br>{lecture}");
                                td.set_inner_html(&lecture_str);
                            }
                        }
                    }
                    7 => {
                        if !row.friday_review2.is_empty() {
                            td.set_class_name("fridayreviewcolumn1");
                            td.set_inner_html(
                                get_review_col(&row.friday_review2, row.day).as_str(),
                            );
                        } else {
                            td.set_class_name("vocnotescolumn");
                            if let Some(voc) = row.voc_notes.as_ref() {
                                let v = format!("Vocabulary Notes<br>{voc}");
                                td.set_inner_html(&v);
                            }
                        }
                    }
                    8 => {
                        td.set_class_name("statscolumn");
                        let v = row.get_stats();
                        let o = format!("{v:?}");
                        td.set_inner_html(&o);
                    }
                    _ => (),
                }
                if col == 2 && row.day < 1 {
                    let _ = td.set_attribute("colspan", "8");
                    if let Some(o) = row.other.clone() {
                        td.set_inner_html(&o);
                    }
                    break;
                }
            }
        }
    }
}

fn get_noon_optional_col(
    title1: &Option<String>,
    fac1: &Option<String>,
    title2: &Option<String>,
    fac2: &Option<String>,
) -> String {
    let mut title = String::from("");
    if let Some(t) = title1
        && let Some(f) = fac1
    {
        title = format!("(optional)<br>{t} - {f}");
    }
    if let Some(t) = title2
        && let Some(f) = fac2
    {
        title.push_str(format!("<br>{t} - {f}").as_str());
    }
    title
}

fn get_drill_col(fac: &[String]) -> String {
    match fac.len() {
        2 => format!("E - {}<br>F - {}", fac[0], fac[1]),
        3 => format!("E - {}<br>F/G - {}<br>H - {}", fac[0], fac[1], fac[2]),
        _ => String::from(""),
    }
}

fn get_review_col(fac: &[String], day: u32) -> String {
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
