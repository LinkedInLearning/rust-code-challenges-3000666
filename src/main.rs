mod vigenere {
    const ALPHABET: [u8; 26] = *b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const A: u8 = b'A';
    const Z: u8 = b'Z';
    const WRAP: u8 = 26; // ALPHABET.len() as u8

    fn clean_input(input: &str) -> impl Iterator<Item = u8> + '_ {
        input.bytes().filter_map(|x| match x {
            A..=Z => Some(x),
            b'a'..=b'z' => Some(x - (b'a' - A)),
            _ => None,
        })
    }

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let mut key_iter = key.bytes().map(|k| k - A).cycle();

        let encrypted = clean_input(plaintext)
            .map(|x| {
                let offset = key_iter.next().unwrap();
                ((x - A) + offset) % WRAP + A
            })
            .collect();

        String::from_utf8(encrypted).unwrap()
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let mut key_iter = key.bytes().map(|k| k - b'A').cycle();

        let ciphertext = clean_input(ciphertext)
            .map(|x| {
                let offset = key_iter.next().unwrap();
                ((x + WRAP - A) - offset) % WRAP + A
            })
            .collect();

        String::from_utf8(ciphertext).unwrap()
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JRKUC
    ";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("{}", plaintext);
}
