use hex::encode;
use rand::Rng;

pub fn generate_random_hex_string() -> String {
    let mut rng = rand::thread_rng();
    let random_bytes: Vec<u8> = (0..16).map(|_| rng.gen()).collect();
    encode(random_bytes)
}
