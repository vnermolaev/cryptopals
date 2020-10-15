use crate::set1::ch3::{compute_probs, kullback_leibler, SAMPLE};
use crate::ToBytes;
use crate::XOR;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn detect_single_character_xor(path: &str) -> (usize, u8, Vec<u8>) {
    let file = File::open(path).expect("Must read");
    let reader = BufReader::new(file);
    let sample_dist = compute_probs((&SAMPLE).as_ref());

    reader
        .lines()
        .into_iter()
        .map(|line| {
            line.map(|l| l.as_str().hex_to_bytes().expect("must convert"))
                .expect("must convert")
        })
        .enumerate()
        .flat_map(|(pos, test)| (0u8..=255).map(move |key| (pos, key, test.xor(&[key]))))
        .min_by_key(|(_, _, guess)| kullback_leibler(&sample_dist, &guess))
        .expect("Minimum must exist")
}

#[cfg(test)]
mod test {
    use crate::set1::ch4::detect_single_character_xor;

    #[test]
    fn test_detect_single_character_xor() {
        let crack = detect_single_character_xor("data/ch4.txt");

        assert_eq!(crack.2, b"Now that the party is jumping\n");
    }
}
