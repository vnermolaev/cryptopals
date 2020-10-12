use crate::set1::ch3::{break_single_xor, compute_probs, kullback_leibler, SAMPLE};
use crate::XOR;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn break_repeating_xor_base64_file(path: &str) -> Vec<u8> {
    let file = File::open(path).expect("Must read");
    let reader = BufReader::new(file);
    let sample_dist = compute_probs((&SAMPLE).as_ref());

    let content = reader
        .lines()
        .into_iter()
        .map(|line| line.expect("Line must be defined"))
        .collect::<String>();

    let input = base64::decode(content).expect("Must decode");

    break_repeating_xor(&sample_dist, &input)
}

fn break_repeating_xor(sample_dist: &HashMap<u8, f32>, input: &[u8]) -> Vec<u8> {
    guess_key_size(input, 40, 3)
        .iter()
        .map(|&key_size| {
            // knowing the key size, break the code
            break_repeating_xor_with_keysize(sample_dist, input, key_size)
        })
        .min_by_key(|key| {
            let kl = kullback_leibler(sample_dist, &input.xor(key));

            println!(
                "{}\n{}\n{}\n\n",
                kl,
                String::from_utf8(key.clone()).unwrap(),
                String::from_utf8(input.xor(key).clone()).unwrap()
            );

            kl
        })
        .unwrap()
}

/// Returns a list of possible key sizes.
fn guess_key_size(input: &[u8], max_key_len: usize, n_guesses: usize) -> Vec<usize> {
    assert!(max_key_len >= 2);

    let n_chunks = 4;

    let mut dists = (2..=max_key_len)
        .map(|key_size| {
            let chunks = input.chunks(key_size).take(n_chunks).collect::<Vec<_>>();
            // collect distances between different chunks
            // 0 -> 1, 2, 3
            // 1 -> 2, 3
            // 2 -> 3
            let dist = {
                let mut d = 0;
                for i in 0..n_chunks {
                    for j in i + 1..n_chunks {
                        d += hamming_dist(chunks[i], chunks[j]);
                    }
                }
                d
            };

            (
                key_size,
                (((dist as f32) / (key_size as f32)) * 10000f32) as u32,
            )
        })
        .collect::<Vec<_>>();

    dists.sort_by(|a, b| a.1.cmp(&b.1));

    dists
        .into_iter()
        .take(n_guesses)
        .map(|(key_size, _)| key_size)
        .collect()
}

fn break_repeating_xor_with_keysize(
    sample: &HashMap<u8, f32>,
    input: &[u8],
    key_size: usize,
) -> Vec<u8> {
    // Now we have (e for encrypted byte, d for decrypted), let key_size = n
    // chunk 1 of length n: e_(1, 1) e(1, 2) ... e(1, n)
    // chunk 2 of length n: e_(2, 1) e(2, 1) ... e(2, n)
    // ...
    // chunk k of length n: e_(k, 1) e(k, 1) ... e(k, 1)
    //
    // all e_(m, 1) are encrypted under a single xor key.
    // we need to decrypt column by column messages under a single key to reconstruct the full key.

    input
        .chunks(key_size)
        .fold(Vec::new(), |mut transposed, col| {
            if transposed.len() == 0 {
                col.iter().for_each(|b| transposed.push(vec![*b]));
            } else {
                col.iter()
                    .enumerate()
                    .for_each(|(i, b)| transposed[i].push(*b));
            }

            transposed
        })
        .iter()
        .map(|column| break_single_xor(sample, column))
        .collect()
}

fn hamming_dist(t: &[u8], s: &[u8]) -> u32 {
    assert_eq!(t.len(), s.len());

    t.xor(s).iter().fold(0u32, |dist, b| dist + b.count_ones())
}

#[cfg(test)]
mod test {
    use crate::set1::ch3::{compute_probs, SAMPLE};
    use crate::set1::ch6::{
        break_repeating_xor_base64_file, break_repeating_xor_with_keysize, guess_key_size,
        hamming_dist,
    };
    use crate::XOR;

    #[test]
    fn test_hamming_dist() {
        let t = b"this is a test";
        let s = b"wokka wokka!!!";

        assert_eq!(hamming_dist(t, s), 37);
    }

    #[test]
    fn test_select_key_size() {
        let message = b"This is a test message from the select key size test";
        let key = b"secret";
        let input = message.xor(key);

        let key_sizes = guess_key_size(&input, 10, 3);
        println!("{:?}", key_sizes);
        assert!(key_sizes.iter().any(|&ks| ks == key.len()));
    }

    #[test]
    fn test_break_repeating_xor_with_keysize() {
        let message = b"This is a test message sent from the test. It must be crackable. Or is it?";
        let key = b"secret";
        let input = message.xor(key);

        let sample_dist = compute_probs((&SAMPLE).as_ref());

        let crack = break_repeating_xor_with_keysize(&sample_dist, &input, key.len());

        // println!("{}", String::from_utf8(crack.clone()).unwrap());
        assert_eq!(crack, key);
    }

    #[test]
    fn test_break_repeating_xor_base64_file() {
        let crack = break_repeating_xor_base64_file("src/set1/ch6-sample.txt");
        assert_eq!(crack, b"Terminator X: Bring the noise");
    }
}
