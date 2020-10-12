use anyhow::Result;

mod ch1;
mod ch2;
mod ch3;
mod ch4;

/// Convert string of bytes to actual bytes.
///
/// Difference with as_bytes that here each _two_ symbols are treated as a byte.
fn bytes_from_hex_str(hex: &str) -> Result<Vec<u8>> {
    let mut bytes = vec![];

    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..=(i + 1)], 16)?;
        bytes.push(byte);
    }

    Ok(bytes)
}
