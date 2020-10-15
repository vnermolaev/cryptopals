use crate::set1::read_base64_file_content;
use openssl::symm::{decrypt, Cipher};

fn decrypt_aes_ecb(path: &str, key: &[u8]) -> Vec<u8> {
    let content = read_base64_file_content(path);
    assert_eq!(content.len() % key.len(), 0);

    decrypt(Cipher::aes_128_ecb(), key, None, &content).expect("Must decrypt")
}

#[cfg(test)]
mod test {
    use crate::set1::ch7::decrypt_aes_ecb;

    #[test]
    fn test_decrypt_aes_ecb() {
        let key = b"YELLOW SUBMARINE";

        let decrypted = decrypt_aes_ecb("data/ch7.txt", key);
        let excerpt =
            b"I'm back and I'm ringin' the bell \nA rockin' on the mike while the fly girls yell";

        assert_eq!(&decrypted[..excerpt.len()], excerpt);
    }
}
