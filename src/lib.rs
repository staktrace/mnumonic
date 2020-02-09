fn words() -> Vec<&'static str> {
    #[cfg(feature = "en")]
    let list = include_str!("../words/en.txt");

    let words : Vec<&'static str> = list.lines().collect();
    assert_eq!(words.len(), 256);
    // assert!(words.is_sorted()); // nightly only for now, see https://github.com/rust-lang/rust/issues/53485
    words
}

pub fn encode_bytes(bytes: &[u8]) -> Vec<&'static str> {
    let words = words();
    let mut result = Vec::new();
    for b in bytes {
        result.push(words[*b as usize]);
    }
    result
}

pub fn encode_u32(num: u32) -> Vec<&'static str> {
    let mut bytes = vec![
        ((num >> 24) & 0xFF) as u8,
        ((num >> 16) & 0xFF) as u8,
        ((num >> 8) & 0xFF) as u8,
        (num & 0xFF) as u8,
    ];
    // Drop leading zeros
    while bytes.len() > 1 {
        if bytes[0] == 0 {
            bytes.remove(0);
        } else {
            break;
        }
    }
    encode_bytes(&bytes)
}

pub fn decode_bytes(encoded_words: &[&str]) -> Result<Vec<u8>, usize> {
    let mut result = Vec::new();
    let words = words();
    for (i, encoded) in encoded_words.iter().enumerate() {
        let ix = words.binary_search(&encoded.to_lowercase().as_str()).map_err(|_| i)?;
        result.push(ix as u8);
    }
    Ok(result)
}

pub fn decode_u32(encoded_words: &[&str]) -> Result<u32, usize> {
    let bytes = decode_bytes(encoded_words)?;
    if bytes.len() > 4 {
        return Err(4);
    }
    let mut result : u32 = 0;
    for b in bytes {
        result = (result << 8) | (b as u32 & 0xFF);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode_bytes(&[]), Vec::<&str>::new());
        assert_eq!(encode_u32(0xDEADBEEF).join(" "), "sweet pump second tree".to_string());
        assert_eq!(encode_u32(1234).join(" "), "ant star".to_string());
        assert_eq!(encode_u32(5).join(" "), "apple".to_string());
        assert_eq!(encode_u32(0).join(" "), "able".to_string());
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode_bytes(&[]).unwrap(), vec![]);
        assert_eq!(decode_bytes(&["swEET", "pump", "second", "TREE"]).unwrap(), vec![0xDE, 0xAD, 0xBE, 0xEF]);
        assert_eq!(decode_u32(&["ant", "star"]).unwrap(), 1234);
    }

    #[test]
    fn test_decode_failure() {
        assert_eq!(decode_bytes(&["nonsense"]), Err(0));
        assert_eq!(decode_u32(&["sweet", "pump", "second", "tree", "tree"]), Err(4));
    }
}
