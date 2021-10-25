fn sum(numbers: Vec<Option<i32>>) -> i32 {
    numbers.iter()
    .map(|x| x.unwrap_or(0))
    .sum()
}

fn main() {
    println!("");
}


#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum(nn), 0);
}

#[test]
fn no_missing() {
    let nn = vec![Some(1), Some(5), Some(4)];
    assert_eq!(sum(nn), 10);
}

#[test]
fn some_missing() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum(nn), 10);
}

#[test]
fn all_missing() {
    let nn = vec![None, None, None];
    assert_eq!(sum(nn), 0);
}
