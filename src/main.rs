mod run_length_encoding {
    pub fn encode(text: &str) -> String {
        let mut count = 0; 
        let mut prev : Option<char> = None;
        let mut encoded = String::with_capacity(text.len()/2);
        let mut chars = text.chars();
    
        while let Some(c) = chars.next() {
            if prev.is_none() {
                prev = Some(c);
            }
    
            if prev.unwrap() != c || count == 9 {
                encoded.push_str(&format!("{}{}", count, prev.unwrap()));
                count = 0
            }
            prev = Some(c);
            count += 1;
        }
    
        // protect against empty string
        if let Some(prev) = prev {
            encoded.push_str(&format!("{}{}", count, prev));
        }
        encoded
    }
    
    pub fn decode(text: &str) -> String {
        let mut decoded = String::with_capacity(text.len() * 2);
        let mut chars = text.chars();

        while let (Some(n), Some(c)) = (chars.next(), chars.next()) {
            
            let n = n.to_digit(10).unwrap();
            for _ in 0..n {
                decoded.push(c);
            }
        }

        decoded
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

#[test]
fn long_run() {
    use run_length_encoding::*;

    let input = "AAAAA AAAAAAAAAA AAAAAAAAAAAAAAAAAAAA";
    assert_eq!(encode(input), "5A1 9A1A1 9A9A2A");
}

