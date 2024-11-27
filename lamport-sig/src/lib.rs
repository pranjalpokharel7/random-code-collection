use rand::RngCore;
use sha256::digest;

mod hexmap;

#[derive(Debug)]
pub struct Key {
    pub zeros: Vec<String>,
    pub ones: Vec<String>,
}

impl Key {
    pub fn new() -> Self {
        Self {
            zeros: Vec::with_capacity(256),
            ones: Vec::with_capacity(256),
        }
    }

    pub fn bit_choice(&self, bit: u8) -> &Vec<String> {
        match bit {
            0 => &self.zeros,
            1 => &self.ones,
            _ => &self.zeros,
        }
    }
}

pub fn keygen() -> (Key, Key) {
    let mut private_key: Key = Key::new();
    let mut public_key: Key = Key::new();

    let mut rng = rand::thread_rng();
    for _ in 0..256 {
        let z: u32 = rng.next_u32();
        let zero_row = digest(z.to_string());
        public_key.zeros.push(digest(zero_row.clone()));
        private_key.zeros.push(zero_row);

        let o: u32 = rng.next_u32();
        let one_row = digest(o.to_string());
        public_key.ones.push(digest(one_row.clone()));
        private_key.ones.push(one_row);
    }

    (private_key, public_key)
}

pub fn sign(message: &String, private_key: &Key) -> Vec<String> {
    let message_hash = digest(message);
    let mut signature: Vec<String> = Vec::new();
    for (j, c) in message_hash.as_bytes().iter().enumerate() {
        let num = hexmap::hex_map_u8(c);
        for i in 0..4 {
            let k = (num >> (3 - i)) & 1;
            signature.push(private_key.bit_choice(k)[j * 4 + i].clone());
        }
    }
    signature
}

pub fn verify(message: &String, public_key: &Key, signature: &Vec<String>) -> bool {
    let message_hash = digest(message);
    for (j, c) in message_hash.as_bytes().iter().enumerate() {
        let num = hexmap::hex_map_u8(c);
        for i in 0..4 {
            let k = (num >> (3 - i)) & 1;
            let v = digest(signature[j * 4 + i].clone());
            if public_key.bit_choice(k)[j * 4 + i] != v {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (private_key, public_key) = keygen();
        let message = String::from("I am Pranjal.");
        let signature = sign(&message, &private_key);
        assert!(verify(&message, &public_key, &signature));
    }
}
