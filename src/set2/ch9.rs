use std::convert::TryFrom;

/// Pads the input from the back inplace to a **multiple** of the specified length with filler.
pub fn pad_inplace(input: &mut Vec<u8>, len_multiple: usize) {
    assert!(!input.is_empty());

    if input.len() == len_multiple {
        return;
    }

    let extra_bytes = len_multiple - input.len() % len_multiple;

    let filler = u8::try_from(extra_bytes).expect("Number of bytes to pad must be of size u8");

    input.append(&mut vec![filler; extra_bytes]);
}

/// Unpads the input from the back inplace.
/// Assumes input was padded correctly.
pub fn unpad_inplace(input: &mut Vec<u8>) {
    assert!(!input.is_empty());

    let extra_bytes = *input.last().unwrap() as usize;

    assert!(input[input.len() - extra_bytes..]
        .iter()
        .all(|b| *b == extra_bytes as u8));

    input.truncate(input.len() - extra_bytes);
}

#[cfg(test)]
mod test {
    use crate::set2::ch9::{pad_inplace, unpad_inplace};

    #[test]
    fn test_pad_inplace() {
        let mut input = b"YELLOW SUBMARINE".to_vec();

        pad_inplace(&mut input, 20);

        assert_eq!(input.len(), 20);
        assert_eq!(input[19], 4);
    }

    #[test]
    fn test_pad_unpad_roundtrip() {
        let original = b"YELLOW SUBMARINE";
        let mut input = original.to_vec();

        pad_inplace(&mut input, 20);
        unpad_inplace(&mut input);

        let input_ref: &[u8] = input.as_ref();

        assert_eq!(original, input_ref);
    }
}
