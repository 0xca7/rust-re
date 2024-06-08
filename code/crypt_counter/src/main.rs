
fn crypt_counter(data: &mut [u8]) {
    data
        .iter_mut()
        .enumerate()
        .for_each(|(i, x)| {
            *x ^= 0xde ^ i as u8;
        });
}

fn main() {
    let mut buffer = std::fs::read("plaintext.txt")
        .expect("failed to read plaintext");
    crypt_counter(&mut buffer );
    println!("encrypted: {:02x?}", buffer);
}
