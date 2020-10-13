#![allow(dead_code)]
use anyhow::Result;

mod set1;

pub trait XOR {
    fn xor(&self, _: &Self) -> Vec<u8>;
    fn xor_inplace(&mut self, _: &Self);
}

impl XOR for [u8] {
    fn xor(&self, t: &[u8]) -> Vec<u8> {
        let mut result = self.to_vec();
        result[..].xor_inplace(t);
        result
    }

    fn xor_inplace(&mut self, t: &[u8]) {
        for chunk in self.chunks_mut(t.len()) {
            let len = chunk.len();
            for (c, &d) in chunk.iter_mut().zip(t[..len].iter()) {
                *c ^= d;
            }
        }
    }
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for &[u8] {
    fn to_hex(&self) -> String {
        self.iter().map(|b| format!("{:02x?}", b)).collect()
    }
}

pub trait ToBytes {
    fn hex_to_bytes(&self) -> Result<Vec<u8>>;
}

impl ToBytes for &str {
    fn hex_to_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes = Vec::new();

        for i in (0..self.len()).step_by(2) {
            let byte = u8::from_str_radix(&self[i..=(i + 1)], 16)?;
            bytes.push(byte);
        }

        Ok(bytes)
    }
}
