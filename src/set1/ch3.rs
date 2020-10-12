use crate::XOR;
use std::collections::HashMap;

pub fn compute_probs(phrase: &[u8]) -> HashMap<u8, f32> {
    let freq = phrase.iter().fold(HashMap::new(), |mut freq, c| {
        if !c.is_ascii() {
            return freq;
        }

        let c = if c.is_ascii_alphabetic() {
            c.to_ascii_lowercase()
        } else if c.is_ascii_control() {
            0
        } else {
            *c
        };

        *freq.entry(c).or_insert(0u32) += 1;
        freq
    });

    freq.into_iter()
        .map(|(k, v)| (k, v as f32 / phrase.len() as f32))
        .collect()
}

pub fn kullback_leibler(sample_dist: &HashMap<u8, f32>, test: &[u8]) -> u32 {
    if !test.is_ascii() {
        return u32::MAX;
    }

    let test_dist = compute_probs(test);

    if !test_dist.keys().all(|k| sample_dist.contains_key(k)) {
        // test contains keys unknown to the sample.
        return u32::MAX;
    }

    (sample_dist.keys().fold(0f32, |kl, c| {
        let q = sample_dist.get(c).expect("Must have own key");
        let p = test_dist.get(c).unwrap_or(&f32::EPSILON);

        kl + (p * (p / q).ln())
    }) * 1000f32) as u32
}

pub fn break_single_xor(sample: &HashMap<u8, f32>, test: &[u8]) -> u8 {
    (0u8..=255)
        .min_by_key(|&key| kullback_leibler(sample, &test.xor(&[key])))
        .unwrap()
}

pub const SAMPLE: &'static str =
    "the quick brown fox jumps over the lazy dog.
    In mathematical statistics, the Kullback–Leibler divergence (also called relative entropy) is a measure
of how one probability distribution is different from a second, reference probability distribution.
Applications include characterizing the relative (Shannon) entropy in information systems, randomness in
continuous time-series, and information gain when comparing statistical models of inference. In contrast to
variation of information, it is a distribution-wise asymmetric measure and thus does not qualify as
a statistical metric of spread - it also does not satisfy the triangle inequality. In the simple case,
a Kullback–Leibler divergence of 0 indicates that the two distributions in question are identical.
In simplified terms, it is a measure of surprise, with diverse applications such as applied statistics,
fluid mechanics, neuroscience and machine learning.
The Kullback–Leibler divergence was introduced by Solomon Kullback and Richard Leibler in 1951 as
the directed divergence between two distributions; Kullback preferred the term discrimination information.
The divergence is discussed in Kullback's 1959 book, Information Theory and Statistics.";

#[cfg(test)]
mod test {
    use crate::set1::bytes_from_hex_str;
    use crate::set1::ch3::{break_single_xor, compute_probs, SAMPLE};
    use crate::XOR;

    #[test]
    fn test_single_byte_xor_cipher() {
        let s = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736".to_string();
        let bytes = bytes_from_hex_str(&s).expect("must convert");

        let sample_dist = compute_probs((&SAMPLE).as_ref());
        let key = break_single_xor(&sample_dist, &bytes);

        // println!("{}", String::from_utf8(bytes.xor(&[key])).unwrap());
        assert_eq!(bytes.xor(&[key]), b"Cooking MC's like a pound of bacon");
    }
}
