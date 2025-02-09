use crate::types::le::encoding::Encoding;

#[cfg_attr(feature = "inline_always", inline(always))]
pub fn str_from_bytes(bytes: &[u8], enc1: &Encoding, enc2: &Option<Encoding>) -> std::io::Result<String> {
    enc1.decode(bytes)
        .or_else(|err| {
            enc2.as_ref()
                .map(|enc| enc.decode(bytes))
                .unwrap_or_else(|| Err(err))
        })
}

#[cfg_attr(feature = "inline_always", inline(always))]
pub fn str_to_bytes(value: &String, enc1: &Encoding, enc2: &Option<Encoding>) -> std::io::Result<Vec<u8>> {
    enc1.encode(value)
        .or_else(|err| {
            enc2.as_ref()
                .map(|enc| enc.encode(value))
                .unwrap_or_else(|| Err(err))
        })
}