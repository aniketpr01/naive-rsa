use crate::rsa::RSA;

mod mulinv;
mod prime;
mod rsa;

fn main() {
    let msg = "Hello world!".to_string();
    println!("Plain text {}", msg);
    let rsa = RSA::new();
    let ciphertext = rsa.encrypt(msg);
    println!("Ciphertext: {:#?}", ciphertext);
    let decrypted = rsa.decrypt(ciphertext);
    println!("Decrypted Ciphertext: {}", decrypted);
}
