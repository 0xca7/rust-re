/// rust reversing - simple encryption with XOR

use std::fs;

/// xor encryption with an iterator
fn crypt(data: &mut [u8]) {
    data
        .iter_mut()
        .for_each(|x| *x ^= 0xde);
}

fn main() {

    let mut data = fs::read("plaintext.txt")
        .expect("failed to read data");

    crypt(&mut data);
    println!("encrypted: {:x?}", data);

    let mut data = fs::read("plaintext.txt.encrypted")
        .expect("failed to read data");

    crypt(&mut data);
    println!("decrypted: {:x?}", data);

    let s = String::from_utf8_lossy(&data);
    println!("decrypted string: {}", s);

}
