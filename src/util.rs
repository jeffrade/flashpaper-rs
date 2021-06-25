use hex::{decode, encode, FromHexError};
use std::env;
use std::env::VarError;

pub fn get_os_env_value(env_var: &str) -> Result<String, VarError> {
    env::var(env_var)
}

pub fn hex_decode(hex_str: &str) -> Result<Vec<u8>, FromHexError> {
    decode(hex_str)
}

pub fn hex_encode(bytes: &[u8]) -> String {
    encode(bytes)
}

/// https://github.com/AndrewPaglusch/FlashPaper/blob/94f269c0be912fa87094671fd8cb486f3f7cc338/includes/functions.php#L129
pub fn hex_urlify(s: &str) -> String {
    s.replace("+", "-").replace("/", "_").replace("=", "#")
}

/// https://github.com/AndrewPaglusch/FlashPaper/blob/94f269c0be912fa87094671fd8cb486f3f7cc338/includes/functions.php#L133
pub fn hex_deurlify(s: &str) -> String {
    s.replace("-", "+").replace("_", "/").replace("#", "=")
}

pub fn append_arrays(a: &[u8], b: &[u8]) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();
    c.append(&mut a.to_vec());
    c.append(&mut b.to_vec());
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_append_arrays() {
        let actual = append_arrays(&[1, 2], &[3, 4, 5]);
        assert_eq!(vec![1, 2, 3, 4, 5], actual);

        let (left, right) = actual.split_at(3);
        assert_eq!([1, 2, 3], left);
        assert_eq!([4, 5], right);
        assert_eq!(vec![1, 2, 3, 4, 5], actual);
    }
}
