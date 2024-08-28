// Inline module - Cara Kedua
pub mod random {
    use rand::Rng;

    pub fn string(length: u32) -> String {
        const CHARSET: &[u8] = "abcdefghijklmnopqrstuvwxyz".as_bytes();
        let mut arr = Vec::new();
        for _ in 0..=length {
            let n = rand::thread_rng().gen_range(0..(CHARSET.len()));
            let char = CHARSET[n];
            arr.push(char);
        }

        std::str::from_utf8(&arr[..]).unwrap().to_string()
    }
}

pub mod password {
    pub fn hash(text: &str) -> String {
        let result = bcrypt::hash(text, bcrypt::DEFAULT_COST).unwrap();
        result
    }

    pub fn is_valid(plain: &str, hashed: &str) -> bool {
        let valid = bcrypt::verify(plain, hashed).unwrap();
        valid
    }
}
