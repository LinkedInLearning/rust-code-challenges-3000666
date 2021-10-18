mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        todo!()
    }
    
    pub fn decode(text: &str) -> String {
        todo!()
    }
}

fn main() {
    // 
}

#[test]
fn abc() {
    use run_length_encoding::*;

    assert_eq!(encode("abc"), "1a1b1c");
}

#[test]
fn round_trip() {
    use run_length_encoding::*;

    let input = "LinkedIn";
    assert_eq!(decode(&encode(input)), input);
}

