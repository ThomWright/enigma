
/// An internal representation of a substitution cipher.
///
/// The numbers 0-25 are used to represent the characters A-Z respectively.
///
/// The array index is the input character, and the value is the output character.
pub type Cipher = [CipherChar; 26];
pub type CipherChar = usize;

const ASCII_UPPERCASE_A: usize = 65;

pub fn from_char(c: char) -> CipherChar {
    use std::ascii::AsciiExt;
    assert!(c.is_ascii() && c.is_alphabetic(), "Char must be ascii and alphabetic");

    c.to_ascii_uppercase() as CipherChar - ASCII_UPPERCASE_A
}

pub fn to_char(n: CipherChar) -> char {
    char::from((n + ASCII_UPPERCASE_A) as u8)
}

pub fn from_string(cipher_str: &str) -> Cipher {
    assert!(cipher_str.len() == 26, "String length must be 26");

    let mut used_chars = [false; 26];
    let mut cipher = [0; 26];

    for (i, v) in cipher_str.chars().enumerate() {
        let cipher_char = from_char(v);
        assert!(
            used_chars[cipher_char] == false,
            "Cipher string must not contain duplicate characters"
        );
        used_chars[cipher_char] = true;
        cipher[i] = cipher_char;
    };
    cipher
}

pub fn is_pairs(cipher: Cipher) -> bool {
    for (i, v) in cipher.iter().enumerate() {
        if cipher[*v] != i {
            return false
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_conversions() {
        assert_eq!('A', to_char(from_char('a')));
        assert_eq!('A', to_char(from_char('A')));
    }

    #[test]
    #[should_panic(expected = "must be ascii and alphabetic")]
    fn from_non_ascii_char() {
        from_char('Δ');
    }

    #[test]
    #[should_panic(expected = "must be ascii and alphabetic")]
    fn from_nonalphabetic_char() {
        from_char('5');
    }

    #[test]
    fn cipher_creation() {
        assert_eq!(
            [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25],
            from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ")
        );
    }

    #[test]
    #[should_panic(expected = "must not contain duplicate characters")]
    fn cipher_creation_duplicate_chars() {
        from_string("ABCDEFGHIJKAMNOPQRSTUVWXYZ");
    }

    #[test]
    fn cipher_of_pairs() {
        assert!(is_pairs(from_string("YRUHQSLDPXNGOKMIEBFZCWVJAT")));
        assert!(!is_pairs(from_string("RYUHQSLDPXNGOKMIEBFZCWVJAT")));
    }
}
