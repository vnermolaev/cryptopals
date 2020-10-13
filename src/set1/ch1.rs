use crate::ToBytes;
use anyhow::Result;

fn ch1_convert_hex_to_base64(hex: &str) -> Result<String> {
    Ok(base64::encode(&hex.hex_to_bytes()?))
}

#[cfg(test)]
mod test {
    use crate::set1::ch1::ch1_convert_hex_to_base64;
    use anyhow::Result;

    #[test]
    fn test_ch1_convert_hex_to_base64() -> Result<()> {
        let test = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
        let result = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();

        let attempt = ch1_convert_hex_to_base64(&test)?;
        assert_eq!(attempt, result);

        Ok(())
    }
}
