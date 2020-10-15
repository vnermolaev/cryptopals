use crate::ToBytes;
use anyhow::Result;

fn fixed_xor(s1: &str, s2: &str) -> Result<String> {
    let xored = s1
        .hex_to_bytes()?
        .iter()
        .zip(s2.hex_to_bytes()?.iter())
        .map(|(&b1, &b2)| format!("{:x}", b1 ^ b2))
        .collect::<String>();

    Ok(xored)
}

#[cfg(test)]
mod test {
    use crate::set1::ch2::fixed_xor;
    use anyhow::Result;

    #[test]
    fn test_ch2_fixed_xor() -> Result<()> {
        let s1 = "1c0111001f010100061a024b53535009181c".to_string();
        let s2 = "686974207468652062756c6c277320657965".to_string();
        let result = "746865206b696420646f6e277420706c6179".to_string();

        let attempt = fixed_xor(&s1, &s2)?;
        assert_eq!(attempt, result);

        Ok(())
    }
}
