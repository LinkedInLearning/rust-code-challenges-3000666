// TODO: import the necessary dependencies

struct ImportantEvent {
    // TODO: define data structure
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    // TODO: implement trait
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: Local.ymd(2020, 12, 25),
    };
    
    if missed_christmas.is_passed() {
        println!("oh well, maybe next year");
    } else {
        println!("☃︎");
    }
}

#[test]
fn in_past() {
    use chrono::Duration;

    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::today() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    use chrono::Duration;
    
    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::today() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}

