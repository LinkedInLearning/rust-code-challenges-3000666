use chrono::{Local, NaiveDate, TimeZone};

fn is_year(field: &str) -> bool {
    field.len() == 4 && field.chars().all(|x| x.is_ascii_digit())
}

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`. 
fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let text = text.trim();

    // check that there are numbers
    if !text.bytes().any(|x| x.is_ascii_digit()) {
        return None;
    } 

    // allow any known delimiter
    let fields: Vec<_> = text
        .split(['/', '-', '.', ' '].as_slice())
        .collect();

    let mut year = None;
    let mut month = None;
    let mut day = None;

    for field in fields.iter() {
        if field.len() < 3 {
            continue;
        }

        let m = match &field.to_lowercase()[..3] {
            "jan" => 1,
            "feb" => 2,
            "mar" => 3,
            "apr" => 4,
            "may" => 5,
            "jun" => 6,
            "jul" => 7,
            "aug" => 8,
            "sep" => 9,
            "oct" => 10,
            "nov" => 11,
            "dec" => 12,
            _ => continue,
        };

        month = Some(m)
    }

    for field in fields.iter() {
        if is_year(field) {
            year = field.parse::<i32>().ok();
            continue;
        }

        if month.is_some() {
            day = field.parse::<u32>().ok();
        } else {
            month = field.parse::<u32>().ok();
        }

    }

    match (year, month, day) {
        (Some(y), Some(m), None) => NaiveDate::from_ymd_opt(y, m, 1),
        (Some(y), Some(m), Some(d)) => NaiveDate::from_ymd_opt(y, m, d),
        _ => None,
    }
}

fn main() {
    let dates = [
        "2002 Feb 02",
        "2010-12-11",
        "1999/March/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }

}

#[test]
fn ymd_hyphen() {
    assert_eq!(flexible_date_parse("2010-12-11"), Some(NaiveDate::from_ymd(2010, 12, 11)))
}

#[test]
fn ymd_slash() {
    assert_eq!(flexible_date_parse("1999/Mar/02"), Some(NaiveDate::from_ymd(1999, 3, 2)))
}

#[test]
fn dmy_dot() {
    assert_eq!(flexible_date_parse("01.Mar.2021"), Some(NaiveDate::from_ymd(2021, 3, 1)))
}

#[test]
fn mdy_dot() {
    assert_eq!(flexible_date_parse("Apr.05.2021"), Some(NaiveDate::from_ymd(2021, 4, 5)))
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}
