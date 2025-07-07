use jiff::civil::Weekday;
use lgi_schedule::*;
use wasm_bindgen::prelude::*;
use web_sys::Document;
use web_sys::Element;

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    create_table();
    Ok(())
}

#[wasm_bindgen]
pub fn create_table() {
    //let juneteenth = "2025-06-19 08:30[America/New_York]".parse().unwrap();
    //let julyfour = "2025-07-04 08:30[America/New_York]".parse().unwrap();

    let start_date = "2025-06-09";
    let holidays = vec!["2025-06-19", "2025-07-04"];
    let faculty = vec![
        vec!["BP", "EBH", "HH", "JM"],
        vec!["HH", "JM", "EBH", "BP"],
        vec!["EBH", "JM", "HH", "BP"],
        vec!["EBH", "JM", "BP"],
        vec!["JM", "EBH", "BP"],
        vec!["EBH", "JM", "BP"],
        vec!["ABF", "JM", "EBH"],
        vec!["ABF", "JM", "EBH"],
        vec!["ABF", "JM", "EBH"],
        vec!["ABF", "JM", "EBH"],
        vec!["ABF", "JM", "EBH"],
    ];

    let p = Params {
        faculty,
        start_date,
        holidays,
    };

    let summer = create_summer(&p).unwrap();

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let mut table = document.create_element("table").unwrap();

    for row in &summer.days {
        if row.date.weekday() == Weekday::Monday {
            table = document.create_element("table").unwrap();
            table.set_class_name(format!("week-table week{}", row.week).as_str());
            body.append_child(&table).unwrap();

            let tr = document.create_element("tr").unwrap();
            table.append_child(&tr).unwrap();

            let td = document.create_element("td").unwrap();
            tr.append_child(&td).unwrap();
            td.set_inner_html(format!("Week {}", row.week).as_str());
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
                        get_stat_table(&document, &td, row.get_stats());
                        // let v = row.get_stats();
                        // let o = format!("{v:?}");
                        // td.set_inner_html(&o);
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
                        get_stat_table(&document, &td, row.get_stats());
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
                        get_stat_table(&document, &td, row.get_stats());
                    }
                    _ => (),
                }
                if col == 2 && row.day < 1 {
                    let _ = td.set_attribute("colspan", "6");
                    if let Some(o) = row.other.clone() {
                        td.set_inner_html(&o);
                    }

                    let td = document.create_element("td").unwrap();
                    td.set_class_name("statscolumn");
                    get_stat_table(&document, &td, row.get_stats());
                    tr.append_child(&td).unwrap();

                    break;
                }
            }
        }
        if row.date.weekday() == Weekday::Sunday {
            let seqs = summer.get_seqs(row.week);
            let seq_table = document.create_element("table").unwrap();
            seq_table.set_class_name("seqtable");
            body.append_child(&seq_table).unwrap();
            let seq_row = document.create_element("tr").unwrap();
            seq_table.append_child(&seq_row).unwrap();
            let seq_td = document.create_element("td").unwrap();
            seq_row.append_child(&seq_td).unwrap();
            seq_td.set_inner_html("");

            if seqs[0].1.len() == 10 {
                let seq_td = document.create_element("td").unwrap();
                seq_row.append_child(&seq_td).unwrap();
                seq_td.set_inner_html("M1");
                let seq_td = document.create_element("td").unwrap();
                seq_row.append_child(&seq_td).unwrap();
                seq_td.set_inner_html("M2");
            }
            let seq_td = document.create_element("td").unwrap();
            seq_row.append_child(&seq_td).unwrap();
            seq_td.set_inner_html("T1");
            let seq_td = document.create_element("td").unwrap();
            seq_row.append_child(&seq_td).unwrap();
            seq_td.set_inner_html("T2");
            let seq_td = document.create_element("td").unwrap();
            seq_row.append_child(&seq_td).unwrap();
            seq_td.set_inner_html("W1");
            let seq_td = document.create_element("td").unwrap();
            seq_row.append_child(&seq_td).unwrap();
            seq_td.set_inner_html("W2");
            let seq_td = document.create_element("td").unwrap();
            seq_row.append_child(&seq_td).unwrap();
            seq_td.set_inner_html("TH1");
            let seq_td = document.create_element("td").unwrap();
            seq_row.append_child(&seq_td).unwrap();
            seq_td.set_inner_html("TH2");
            let seq_td = document.create_element("td").unwrap();
            seq_row.append_child(&seq_td).unwrap();
            seq_td.set_inner_html("F1");
            let seq_td = document.create_element("td").unwrap();
            seq_row.append_child(&seq_td).unwrap();
            seq_td.set_inner_html("F2");

            for rows in seqs.clone() {
                let seq_row = document.create_element("tr").unwrap();
                seq_table.append_child(&seq_row).unwrap();
                let seq_td = document.create_element("td").unwrap();
                seq_row.append_child(&seq_td).unwrap();
                seq_td.set_inner_html(&rows.0);

                for cols in rows.1 {
                    let seq_td = document.create_element("td").unwrap();
                    seq_row.append_child(&seq_td).unwrap();
                    seq_td.set_inner_html(&cols);
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

fn get_stat_table(document: &Document, td: &Element, stats: Vec<(String, u32)>) {
    let stat_table = document.create_element("table").unwrap();
    stat_table.set_class_name("stattable");
    td.append_child(&stat_table).unwrap();
    //headers
    let stat_row = document.create_element("tr").unwrap();
    stat_table.append_child(&stat_row).unwrap();
    //let stats: Vec<(String, u32)> = row.get_stats();
    for stat_fac in stats.clone() {
        let stat_td = document.create_element("td").unwrap();
        stat_row.append_child(&stat_td).unwrap();
        stat_td.set_inner_html(&stat_fac.0);
    }

    //counts
    let stat_row = document.create_element("tr").unwrap();
    stat_table.append_child(&stat_row).unwrap();
    for stat_fac in stats.clone() {
        let stat_td = document.create_element("td").unwrap();
        stat_row.append_child(&stat_td).unwrap();

        stat_td.set_inner_html(&stat_fac.1.to_string());
    }
}

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
