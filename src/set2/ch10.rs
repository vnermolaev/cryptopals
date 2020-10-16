use crate::set2::ch9::{pad_inplace, unpad_inplace};
use crate::XOR;
use openssl::symm::{decrypt, encrypt, Cipher};

fn encrypt_aes_ecb_block(input: &[u8], key: &[u8], block_size: usize) -> Vec<u8> {
    assert_eq!(input.len(), block_size);

    let mut ciphertext =
        encrypt(Cipher::aes_128_ecb(), key, None, input).expect("OpenSSL must encrypt");

    // ciphertext is padded with the block size
    // let mut out = vec![0; data.len() + t.block_size()];

    ciphertext.truncate(block_size);

    ciphertext
}

fn decrypt_aes_ecb_block(input: &[u8], key: &[u8], block_size: usize) -> Vec<u8> {
    assert_eq!(input.len(), block_size);

    // The OpenSSL call expects a padded cleartext.
    let padded_input = {
        let padding = encrypt_aes_ecb_block(&vec![block_size as u8; block_size], key, block_size);
        let mut i = input.to_vec();
        i.extend_from_slice(&padding);
        i
    };

    decrypt(Cipher::aes_128_ecb(), key, None, &padded_input).expect("OpenSSL must decrypt")
}

fn encrypt_aes_cbc(input: &[u8], key: &[u8], iv: &[u8], block_size: usize) -> Vec<u8> {
    assert_eq!(iv.len(), block_size);

    let mut input = Vec::from(input);

    pad_inplace(&mut input, block_size);

    let ciphertext = input
        .chunks(block_size)
        // Type annotations are required by the compiler.
        .fold(Vec::new(), |mut ciphertext: Vec<Vec<u8>>, block| {
            let previous = ciphertext.last().map_or(iv, |v| v.as_ref());

            let cipher_block = encrypt_aes_ecb_block(&block.xor(previous), key, block_size);

            ciphertext.push(cipher_block);

            ciphertext
        });

    ciphertext.into_iter().flatten().collect::<Vec<_>>()
}

fn decrypt_aes_cbc(input: &[u8], key: &[u8], iv: &[u8], block_size: usize) -> Vec<u8> {
    assert_eq!(iv.len(), block_size);

    let input = Vec::from(input);

    let plaintext =
        input
            .chunks(block_size)
            .fold((Vec::new(), iv), |(mut plaintext, mut previous), block| {
                let plain_block = decrypt_aes_ecb_block(&block, key, block_size).xor(previous);

                plaintext.push(plain_block);
                previous = block;

                (plaintext, previous)
            });

    let mut plaintext = plaintext.0.into_iter().flatten().collect::<Vec<_>>();

    unpad_inplace(&mut plaintext);

    plaintext
}

#[cfg(test)]
mod test {
    use crate::set1::read_base64_file_content;
    use crate::set2::ch10::{
        decrypt_aes_cbc, decrypt_aes_ecb_block, encrypt_aes_cbc, encrypt_aes_ecb_block,
    };

    #[test]
    fn test_aes_ecb_block_encryption_roundtrip() {
        let original = b"YELLOW SUBMARINE";
        let key = b"YELLOW SUBMARINE";
        let block_size = 16;

        let ciphertext = encrypt_aes_ecb_block(original.as_ref(), key.as_ref(), block_size);
        let plaintext = decrypt_aes_ecb_block(ciphertext.as_ref(), key.as_ref(), block_size);
        let plaintext_ref: &[u8] = plaintext.as_ref();

        assert_eq!(original, plaintext_ref);
    }

    #[test]
    fn test_aes_cbc_roundtrip() {
        let original = b"YELLOW SUBMARINE BY THE BEATLES";
        let key = b"YELLOW SUBMARINE";
        let block_size = 16;
        let iv = vec![2; block_size];

        let ciphertext = encrypt_aes_cbc(original.as_ref(), key.as_ref(), &iv, block_size);
        let plaintext = decrypt_aes_cbc(ciphertext.as_ref(), key.as_ref(), &iv, block_size);
        let plaintext_ref: &[u8] = plaintext.as_ref();

        assert_eq!(original, plaintext_ref);
    }

    #[test]
    fn test_decrypt_aes_cbc() {
        let key = b"YELLOW SUBMARINE";
        let block_size = 16;
        let iv = vec![0; block_size];

        let ciphertext = read_base64_file_content("data/ch10.txt");
        let plaintext =
            String::from_utf8(decrypt_aes_cbc(&ciphertext, key.as_ref(), &iv, block_size))
                .expect("Must convert");

        let piece = plaintext.chars().take(33).collect::<String>();

        assert_eq!(piece, "I'm back and I'm ringin' the bell");
    }
}
