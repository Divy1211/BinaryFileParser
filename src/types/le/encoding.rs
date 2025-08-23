use pyo3::prelude::*;

use encoding_rs::WINDOWS_1252;
use pyo3::exceptions::PyValueError;

#[pyclass(module = "bfp_rs.types.le", name = "Encoding", eq)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Encoding {
    ASCII,
    UTF8,
    UTF16,
    UTF32,
    LATIN1,
    WINDOWS1252,
}

impl Encoding {
    pub fn decode(&self, bytes: &[u8]) -> PyResult<String> {
        match self {
            Encoding::ASCII => {
                if bytes.iter().all(|&b| b.is_ascii()) {
                    Ok(String::from_utf8_lossy(bytes).to_string())
                } else {
                    Err(PyValueError::new_err("Invalid ASCII sequence"))
                }
            }
            Encoding::UTF8 => {
                String::from_utf8(bytes.to_vec())
                    .map_err(|_| PyValueError::new_err("Invalid UTF-8 sequence"))
            }
            Encoding::UTF16 => {
                if bytes.len() % 2 != 0 {
                    return Err(PyValueError::new_err("UTF-16 requires even-length byte sequences"));
                }
                let utf16_units: Vec<u16> = bytes
                    .chunks_exact(2)
                    .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                    .collect();

                String::from_utf16(&utf16_units)
                    .map_err(|_| PyValueError::new_err("Invalid UTF-16 sequence"))
            }
            Encoding::UTF32 => {
                if bytes.len() % 4 != 0 {
                    return Err(PyValueError::new_err("UTF-32 requires byte length to be a multiple of 4"));
                }
                bytes
                    .chunks_exact(4)
                    .map(|chunk| u32::from_le_bytes(chunk.try_into().expect("Infallible")))
                    .map(|codepoint| {
                        char::from_u32(codepoint)
                            .ok_or(PyValueError::new_err("Invalid UTF-32 sequence"))
                    })
                    .collect()
            }
            Encoding::LATIN1 => {
                Ok(bytes.iter().map(|&b| b as char).collect())
            }
            Encoding::WINDOWS1252 => {
                let (cow, _, had_errors) = WINDOWS_1252.decode(bytes);
                if had_errors {
                    Err(PyValueError::new_err("Invalid Windows-1252 sequence"))
                } else {
                    Ok(cow.into_owned())
                }
            }
        }
    }

    pub fn encode(&self, text: &String, buffer: &mut Vec<u8>) -> PyResult<()> {
        match self {
            Encoding::ASCII => {
                if text.chars().all(|c| c.is_ascii()) {
                    Ok(buffer.extend_from_slice(text.as_bytes()))
                } else {
                    Err(PyValueError::new_err("String contains chars out of ASCII range"))
                }
            }
            Encoding::UTF8 => Ok(buffer.extend_from_slice(text.as_bytes())),
            Encoding::UTF16 => Ok(buffer.extend(
                text.encode_utf16()
                    .flat_map(|c| c.to_le_bytes())
            )),
            Encoding::UTF32 => Ok(buffer.extend(
                text.chars()
                    .flat_map(|c| (c as u32).to_le_bytes())
            )),
            Encoding::LATIN1 => {
                if text.chars().all(|c| (c as u32) <= 0xFF) {
                    Ok(buffer.extend(text.chars().map(|c| c as u8)))
                } else {
                    Err(PyValueError::new_err("String contains chars out of Latin-1 range"))
                }
            }
            Encoding::WINDOWS1252 => {
                let (bytes, _, had_errors) = WINDOWS_1252.encode(text);
                if had_errors {
                    Err(PyValueError::new_err("Windows-1252 Encoding Error"))
                } else {
                    Ok(buffer.extend_from_slice(&bytes))
                }
            }
        }
    }
}
