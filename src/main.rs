fn median(a: Vec<f32>) -> Option<f32> {
  todo!();
}


fn main() {
    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}




#[test]
fn empty_input() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}


#[test]
fn one_two_five() {
    let input = vec![1.0, 2.0, 5.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}