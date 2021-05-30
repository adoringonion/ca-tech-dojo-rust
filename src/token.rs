use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
const TOKEN_LENGTH: usize = 20;

pub struct Token {
    value: String,
}

impl Token {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let value: String = (0..TOKEN_LENGTH)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        return Token { value };
    }

    pub fn get(self) -> String {
        self.value
    }
}
