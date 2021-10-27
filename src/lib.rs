/// Return the list of words used for doing conversion. The words are
/// read from a source file at compile time. The word list must have
/// exactly 256 words, all in lowercase, and sorted alphabetically so
/// that the list can be binary-searched. Care should be taken to avoid
/// words that are commonly mis-spelled or that may have multiple valid
/// spellings.
fn words() -> Vec<&'static str> {
    #[cfg(feature = "en")]
    let list = include_str!("../words/en.txt");

    let words: Vec<&'static str> = list.lines().collect();
    assert_eq!(words.len(), 256);
    // assert!(words.is_sorted()); // nightly only for now, see https://github.com/rust-lang/rust/issues/53485
    for word in &words {
        assert!(word.len() < 7);
    }
    words
}

/// Encode an array of bytes into a corresponding vector of human readable words.
/// The output vector will be the same length as the input vector.
pub fn encode_bytes(bytes: &[u8]) -> Vec<&'static str> {
    let words = words();
    let mut result = Vec::new();
    for b in bytes {
        result.push(words[*b as usize]);
    }
    result
}

/// Encode a u32 into a vector of words. The output vector will have at least one
/// entry, and no more than four entries; each entry is a single human-language word.
/// Each byte of the u32 is converted to a word, with leading zero bytes dropped.
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

/// Same as encode_u32, except this also joins the individual words into a
/// phrase. The words are joined using an ascii space character as delimiter.
pub fn encode_u32_joined(num: u32) -> String {
    encode_u32(num).join(" ")
}

/// Decode a vector of words (as produced by `encode_bytes`) back into the
/// vector of bytes that produced them. In case of error (e.g. the provided
/// vector has an invalid word), the index of the invalid entry is returned
/// as an `Err`. Note that the words are lowercased per Unicode rules before
/// decoding. This allows for scenarios where the human transcribing the
/// words mixes up the case of the words, or some other system's autocorrect
/// mutates the case.
pub fn decode_bytes(encoded_words: &[&str]) -> Result<Vec<u8>, usize> {
    let mut result = Vec::new();
    let words = words();
    for (i, encoded) in encoded_words.iter().enumerate() {
        let ix = words
            .binary_search(&encoded.to_lowercase().as_str())
            .map_err(|_| i)?;
        result.push(ix as u8);
    }
    Ok(result)
}

/// Decode a vector of words (as produced by `encode_u32`) back into the
/// u32 that produced them. If the provided word vector has more than 4
/// words, then an `Err(4)` is produced; if any the words are invalid, then
/// an `Err` is produced with the index of the bad entry in the input vector.
pub fn decode_u32(encoded_words: &[&str]) -> Result<u32, usize> {
    let bytes = decode_bytes(encoded_words)?;
    if bytes.len() > 4 {
        return Err(4);
    }
    let mut result: u32 = 0;
    for b in bytes {
        result = (result << 8) | (b as u32 & 0xFF);
    }
    Ok(result)
}

/// Decode a phrase (as produced by `encode_u32_joined`). Basically the
/// same as `decode_u32` except it splits the phrase on whitespace to generate
/// the words to decode.
pub fn decode_u32_joined(joined: &str) -> Result<u32, usize> {
    decode_u32(&joined.split_whitespace().collect::<Vec<&str>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(encode_bytes(&[]), Vec::<&str>::new());
        assert_eq!(
            encode_u32(0xDEADBEEF).join(" "),
            "table potato school true".to_string()
        );
        assert_eq!(encode_u32(1234).join(" "), "ant stamp".to_string());
        assert_eq!(encode_u32(5).join(" "), "apple".to_string());
        assert_eq!(encode_u32(0).join(" "), "able".to_string());
    }

    #[test]
    fn test_decode() {
        assert_eq!(decode_bytes(&[]).unwrap(), vec![]);
        assert_eq!(
            decode_bytes(&["taBLE", "potato", "school", "TRUE"]).unwrap(),
            vec![0xDE, 0xAD, 0xBE, 0xEF]
        );
        assert_eq!(decode_u32(&["ant", "stamp"]).unwrap(), 1234);
        assert_eq!(
            decode_u32_joined("table potato school true").unwrap(),
            0xDEADBEEF
        );
    }

    #[test]
    fn test_decode_failure() {
        assert_eq!(decode_bytes(&["nonsense"]), Err(0));
        assert_eq!(
            decode_u32(&["table", "potato", "school", "true", "true"]),
            Err(4)
        );
    }
}
