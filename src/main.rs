use time;
use time::Date;
use time::error::Parse;

fn parse_date(text: &str) -> Result<Date, Parse> {
    todo!();
}

fn weeks_between(earlier: Date, later: Date) -> i64 {
    todo!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let decades = (1900..)
        .step_by(10)
        .take_while(|x| *x < 2100);
    
    println!("weeks between:");
    for decade in decades {
        let start = format!("{}-01-01", decade);
        let end = format!("{}-12-31", decade+9);
        let dates = (parse_date(&start)?, parse_date(&end)?);
        let weeks = weeks_between(-dates.0, dates.1);

        println!("{} and {}: {}", decade, decade+10, weeks);

    }

    Ok(())
}

#[test]
fn one_week() {
    let t1 = parse_date("2010-01-01").unwrap();
    let t2 = parse_date("2010-01-08").unwrap();
    assert_eq!(weeks_between(t1, t2), 1);
}

