/// Function to transpose chacters: {'a'..'z'} <--> {'z'..'a'}
fn character_transpose(c: char) -> char {
    const ASCII_HEX_START: u8 = b'a';
    const ASCII_HEX_END: u8 = b'z';

    (ASCII_HEX_START + ASCII_HEX_END - (c as u8)) as char
}

/// Convert function: both encode/decode.
fn convert(string: &str) -> impl Iterator<Item = char> + '_ {
    string
        .chars()
        .filter(|c| c.is_ascii_digit() || c.is_ascii_alphabetic())
        .enumerate()
        .map(move |(_i, c)| {
            if c.is_ascii_alphabetic() {
                character_transpose(c.to_ascii_lowercase())
            } else {
                c
            }
        })
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    convert(plain)
        .enumerate()
        .fold(String::new(), |mut accumulator, (idx, c)| {
            if idx != 0 && idx % 5 == 0 {
                accumulator.push(' ');
            }
            accumulator.push(c);
            accumulator
        })
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    convert(cipher).collect()
}
