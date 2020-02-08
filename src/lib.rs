fn words() -> Vec<&'static str> {
    #[cfg(feature = "en")]
    let list = include_str!("../words/en.txt");

    let words : Vec<&'static str> = list.lines().collect();
    assert_eq!(words.len(), 256);
    words
}

pub fn encode(num: u32) -> Vec<&'static str> {
    let words = words();
    let mut result = Vec::new();
    let byte3 = ((num >> 24) & 0xFF) as usize;
    let byte2 = ((num >> 16) & 0xFF) as usize;
    let byte1 = ((num >> 8) & 0xFF) as usize;
    let byte0 = (num & 0xFF) as usize;
    if byte3 != 0 {
        result.push(words[byte3]);
    }
    if byte2 != 0 || !result.is_empty() {
        result.push(words[byte2]);
    }
    if byte1 != 0 || !result.is_empty() {
        result.push(words[byte1]);
    }
    result.push(words[byte0]);
    result
}

pub fn encode_str(num: u32) -> String {
    encode(num).join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(encode_str(0xDEADBEEF), "tactics ringbolt skydive uncut");
    }
}
