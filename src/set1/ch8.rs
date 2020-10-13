use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_aes_ecb(encrypted: &str, key_size: usize) -> bool {
    let encrypted = encrypted.as_bytes();
    let mut blocks = encrypted.chunks(2 * key_size).collect::<Vec<_>>();

    let full_len = blocks.len();

    blocks.sort();
    blocks.dedup();

    blocks.len() != full_len
}

fn detect_aes_ecb(path: &str, key_size: usize) -> Option<Vec<AesEcb>> {
    let file = File::open(path).expect("Must read");
    let reader = BufReader::new(file);

    reader
        .lines()
        .into_iter()
        .map(|line| line.expect("Line must be defined"))
        .find(|line| is_aes_ecb(line, key_size))
        .map(|aes_ecb_line| {
            let n_blocks = aes_ecb_line.len() / (2 * key_size);

            aes_ecb_line
                .as_bytes()
                .chunks(2 * key_size)
                .enumerate()
                .fold(HashMap::new(), |mut index, (pos, block)| {
                    index
                        .entry(Vec::from(block))
                        .or_insert_with(Vec::new)
                        .push(pos);
                    index
                })
                .into_iter()
                .fold(
                    vec![AesEcb::Ref(n_blocks as u32 + 1); n_blocks],
                    |mut acc, (block, positions)| {
                        let root = positions[0];
                        let s = String::from_utf8(block).unwrap();
                        acc[root] = AesEcb::Block(s);
                        positions[1..]
                            .iter()
                            .for_each(|&pos| acc[pos] = AesEcb::Ref(root as u32));
                        acc
                    },
                )
        })
}

#[derive(Debug, Clone)]
enum AesEcb {
    Block(String),
    Ref(u32),
}

#[cfg(test)]
mod test {
    use crate::set1::ch8::{detect_aes_ecb, AesEcb};

    #[test]
    fn test_detect_aes_ecb() {
        let detected = detect_aes_ecb("src/set1/ch8-sample.txt", 16)
            .expect("There must be an AES ECB encrypted line");

        println!("{:#?}", detected);

        #[allow(unused_variables)]
        {
            let pattern = AesEcb::Block("08649af70dc06f4fd5d2d69c744cd283".to_string());
            assert!(matches!(&detected[2], pattern));
        }
    }
}
