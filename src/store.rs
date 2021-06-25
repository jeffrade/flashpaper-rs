use crate::crypto;
use crate::db;
use crate::util;

pub fn save(message: Vec<u8>, static_key: &[u8]) -> String {
    let id = crypto::create_rand_bytes(8, true);
    let iv = crypto::create_rand_iv();
    let aes_key = crypto::create_rand_bytes(32, true);

    let k: Vec<u8> = util::append_arrays(&id, &aes_key);
    let hash: String = bcrypt::hash(&k, bcrypt::DEFAULT_COST).expect("Could not hash k!");

    let message_ciphertext =
        crypto::encrypt(&aes_key, &iv, &message).expect("Could not encrypt message!");
    let encrypted_secret = crypto::encrypt(&static_key, &iv, &message_ciphertext)
        .expect("Could not encrypt message_ciphertext!");
    let _ = db::store(
        &util::hex_encode(&id),
        &util::hex_encode(&iv),
        &hash,
        &util::hex_encode(&encrypted_secret),
    );

    util::hex_urlify(&util::hex_encode(&k))
}

pub fn retrieve(k_str: &str, static_key: &[u8]) -> Option<String> {
    let k = match util::hex_decode(&util::hex_deurlify(k_str)).ok() {
        Some(k) => k,
        None => {
            println!("hexadecimal key could not be decoded!");
            return None;
        }
    };

    if k.len() != 40 {
        println!("key length should be 40 bytes!");
        return None;
    }

    let (id_vec, aes_key) = k.split_at(8);
    let id = util::hex_encode(id_vec);

    match db::get(&id) {
        Ok(record) => {
            let iv = util::hex_decode(&record.iv).expect("hexadecimal iv could not be decoded!");
            let hash = &record.hash;
            if hash.is_empty() {
                println!("Empty hash, that id doesn't exist!");
                return None;
            }

            let secret =
                util::hex_decode(&record.secret).expect("hexadecimal secret could not be decoded!");

            match bcrypt::verify(&k, hash) {
                Ok(true) => {
                    let message_ciphertext = crypto::decrypt(static_key, &iv, &secret)
                        .expect("Could not decrypt secret!");
                    let message_vec = crypto::decrypt(&aes_key, &iv, &message_ciphertext)
                        .expect("Could not decrypt ciphertext!");
                    match std::str::from_utf8(&message_vec) {
                        Ok(message) => {
                            if db::delete(&id).is_ok() {
                                Some(message.to_owned())
                            } else {
                                None
                            }
                        }
                        Err(err) => {
                            println!("Could not convert bytes to String! {:?}", err);
                            None
                        }
                    }
                }
                Ok(false) => {
                    println!("bcrypt hash is not equal!");
                    None
                }
                Err(err) => {
                    println!("Error trying to verify hash! {:?}", err);
                    None
                }
            }
        }
        Err(err) => {
            println!("Error trying to get entry from database! {:?}", err);
            None
        }
    }
}
