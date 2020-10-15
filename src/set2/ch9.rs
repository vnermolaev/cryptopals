/// Pads from the back input inplace to a **multiple** of the specified length with filler.
fn pad_inplace(input: &mut Vec<u8>, len_multiple: usize, filler: u8) {
    if input.len() == len_multiple {
        return;
    }

    let extra_bytes = len_multiple - input.len() % len_multiple;

    input.append(&mut vec![filler; extra_bytes]);
}

#[cfg(test)]
mod test {
    use crate::set2::ch9::pad_inplace;

    #[test]
    fn test_pad_inplace() {
        let mut input = b"YELLOW SUBMARINE".to_vec();

        pad_inplace(&mut input, 20, 4);

        assert_eq!(input.len(), 20);
        assert_eq!(input[19], 4);
    }
}
