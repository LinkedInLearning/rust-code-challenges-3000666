mod vigenere {
    pub fn encrypt(plaintext: &str, key: &str) -> String {
        String::new() // Optional
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        todo!()
    }
}

fn main() {
    let key = "WHYRUST";
    let ciphertext = "
    PVCDJG
    PAYCMY
    JR KUC
    ";
    let plaintext = vigenere::decrypt(&ciphertext, key);

    println!("{}", plaintext);
}
