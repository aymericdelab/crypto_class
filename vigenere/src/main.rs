fn main() {
    let plaintext: &str = "ATTACKATDOWN";
    let key: &str = "LEMON";

    // creates keystream
    let keystream = key_stream(plaintext, key);
    let encoding = encode(plaintext, keystream.as_str());

    println!("{}", encoding);

    let decoding = decode(encoding.as_str(), keystream.as_str());
    println!("{}", decoding);
}

fn key_stream(plaintext: &str, key: &str) -> String {
    let quotient = plaintext.len() % key.len();
    key.repeat(quotient + 1)
}

fn encode(plaintext: &str, keystream: &str) -> String {
    let mut result = "".to_string();

    plaintext.bytes().enumerate().for_each(|(x, i)| {
        let byte = keystream.as_bytes()[x] + i - 65;
        if byte <= 90 {
            result.push((byte) as char)
        } else {
            result.push((byte - 25) as char)
        }
    });

    return result;
}

fn decode(encoding: &str, keystream: &str) -> String {
    let mut result = "".to_string();

    encoding.bytes().enumerate().for_each(|(x, i)| {
        let byte = i + 65 - keystream.as_bytes()[x];
        if byte >= 65 {
            result.push((byte) as char)
        } else {
            result.push((byte + 25) as char)
        }
    });

    return result;
}
