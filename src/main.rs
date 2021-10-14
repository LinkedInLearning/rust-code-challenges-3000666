fn unique(a: Vec<f32>) -> Vec<f32> {
    todo!();
}

// advanced 1: use generic types
// fn unique(a: Vec<T>) -> Vec<T> {
//     todo!();
// }

// advanced 2: keep items in order
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

// advanced 3: use iterators
// fn unique(a: Iterable<T>) -> Vec<T> {
//     todo!();
// }

fn main() {
    let input = vec![2.0, 1.0, 1.0];
    let answer = unique(input);
    println!("unique items -> {:?}", answer)
}


#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = vec![1.0, 4.0, 5.0];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = vec![1.0, 5.0, 2.0];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}


#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1.0, 5.0, 2.0, 2.0, 1.0];
    let expected_output = vec![1.0, 5.0, 2.0];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1.0, 5.0, 2.0, 2.0, 1.0];
    input.sort_by(|x,y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1.0, 2.0, 5.0];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
