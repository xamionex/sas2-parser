use std::io::{self, Read, Write};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid save version: {0}")]
    InvalidVersion(i32),
    #[error("Hash mismatch")]
    HashMismatch,
    #[error("Invalid data: {0}")]
    InvalidData(String),
}

/// Read a 7‑bit encoded integer (as used by BinaryReader.WriteString).
pub fn read_7bit_encoded_int<R: Read>(reader: &mut R) -> io::Result<u32> {
    let mut value = 0u32;
    let mut shift = 0;
    loop {
        let mut byte = [0];
        reader.read_exact(&mut byte)?;
        let b = byte[0];
        value |= ((b & 0x7F) as u32) << shift;
        if b & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift > 28 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid 7-bit encoded int",
            ));
        }
    }
    Ok(value)
}

/// Write a 7‑bit encoded integer.
pub fn write_7bit_encoded_int<W: Write>(writer: &mut W, mut value: u32) -> io::Result<()> {
    while value >= 0x80 {
        writer.write_all(&[((value & 0x7F) | 0x80) as u8])?;
        value >>= 7;
    }
    writer.write_all(&[value as u8])
}

/// Read a length‑prefixed UTF‑8 string (as written by BinaryWriter.WriteString).
pub fn read_string<R: Read>(reader: &mut R) -> io::Result<String> {
    let len = read_7bit_encoded_int(reader)? as usize;
    let mut buf = vec![0u8; len];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Write a UTF‑8 string with length prefix (as expected by BinaryReader).
pub fn write_string<W: Write>(writer: &mut W, s: &str) -> io::Result<()> {
    let bytes = s.as_bytes();
    write_7bit_encoded_int(writer, bytes.len() as u32)?;
    writer.write_all(bytes)
}

/// XOR every byte in a slice with a given value.
pub fn xor_data(data: &mut [u8], xor_value: i32) {
    let xor = xor_value as u8;
    for byte in data.iter_mut() {
        *byte ^= xor;
    }
}
