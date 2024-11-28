use key::Key;
use sha256::digest;

mod hexmap;
mod key;

fn generate_and_populate_keys(
    rng: &mut impl rand::Rng,
    private: &mut Vec<String>,
    public: &mut Vec<String>,
) {
    let value = rng.next_u32();
    let row = digest(value.to_string());
    private.push(row.clone());
    public.push(digest(row));
}

pub fn keygen() -> (Key, Key) {
    let mut private_key: Key = Key::new();
    let mut public_key: Key = Key::new();

    let mut rng = rand::thread_rng();
    for _ in 0..256 {
        generate_and_populate_keys(&mut rng, &mut private_key.zeros, &mut public_key.zeros);
        generate_and_populate_keys(&mut rng, &mut private_key.ones, &mut public_key.ones);
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
            let offset = j * 4 + i;
            signature.push(private_key[k][offset].clone());
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
            let offset = j * 4 + i;
            let v = digest(signature[offset].clone());
            if public_key[k][offset] != v {
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
    fn assert_signature_signing_verification() {
        let (private_key, public_key) = keygen();
        let message = String::from("I am Pranjal.");
        let signature = sign(&message, &private_key);
        assert!(verify(&message, &public_key, &signature));
    }
}
