fn info(a: &T) {
    todo!();
}

fn main() {
    let a = "?";
    let b = "?".to_string();
    info(&a);
    info(&b);
}


#[test]
fn str() {
    let input = "Rust";
    info(&input);
}

#[test]
fn string() {
    let input = String::from("Rust");
    info(&input);
}

// #[test]
// fn chars() {
//     let input = 'r';
//     info(&input);
// }

// #[test]
// fn cstring() {
//     use std::ffi::{CString};
//     let input = CString::new("Rust").unwrap();
//     info(&input);
// }

// #[test]
// fn path() {
//     use std::path::Path;
//     let input = Path::new("/tmp/rust");
//     info(input);
// }
