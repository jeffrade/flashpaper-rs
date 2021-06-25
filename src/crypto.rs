use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::{Aes256Gcm, Key, Nonce};

/// https://github.com/AndrewPaglusch/FlashPaper/blob/94f269c0be912fa87094671fd8cb486f3f7cc338/includes/functions.php#L141-L144
/// $id = random_str(8); bytes
/// $iv = random_str(16); bytes
/// $key = random_str(32); bytes
/// NOTE: aes_gcm::Nonce requires 12 bytes, not 16.
pub fn create_rand_bytes(size: u8, disallow_null: bool) -> Vec<u8> {
    let mut index = 0;
    let mut rand_bytes: Vec<u8> = Vec::new();
    while index < size {
        let mut byte = rand::random::<u8>();
        // https://github.com/Keats/rust-bcrypt/issues/17
        if disallow_null {
            while byte == 0u8 {
                byte = rand::random::<u8>();
            }
        }
        rand_bytes.push(byte);
        index += 1;
    }
    rand_bytes
}

pub fn create_rand_iv() -> Vec<u8> {
    create_rand_bytes(12, false)
}

/// https://docs.rs/aes-gcm/0.9.2/aes_gcm/index.html#usage
pub fn encrypt(key_b: &[u8], iv_b: &[u8], message_b: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let cipher = Aes256Gcm::new(Key::from_slice(key_b));
    let nonce = Nonce::from_slice(iv_b);
    cipher.encrypt(nonce, message_b.as_ref())
}

/// https://docs.rs/aes-gcm/0.9.2/aes_gcm/index.html#usage
pub fn decrypt(key_b: &[u8], iv_b: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, aes_gcm::Error> {
    let cipher = Aes256Gcm::new(Key::from_slice(key_b));
    let nonce = Nonce::from_slice(iv_b);
    cipher.decrypt(nonce, ciphertext)
}
