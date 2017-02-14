use cipher;
use cipher::Cipher;
use cipher::CipherChar;
use cipher::is_pairs;

// A Reflector is a substitution cipher comprising 13 pairs.
//
// For example, an `AB` pair where `A -> B` and `B -> A`.
pub struct Reflector {
    cipher: Cipher,
}

impl Reflector {
    pub fn new(cipher: Cipher) -> Reflector {
        assert!(is_pairs(cipher), "Reflector cipher must be in pairs");
        Reflector {
            cipher: cipher,
        }
    }

    pub fn sub(&self, c: CipherChar) -> CipherChar {
        self.cipher[c]
    }
}

pub struct Reflectors {
    pub b: Reflector,
}

impl Reflectors {
    pub fn new() -> Reflectors {
        Reflectors {
            b: Reflector {
                cipher: cipher::from_string("YRUHQSLDPXNGOKMIEBFZCWVJAT")
            }
        }
    }
}

// TODO?
// fn from_pairs(pairs: &str) -> String {
//
// }

#[cfg(test)]
mod tests {
    use super::*;
    use cipher as Cipher;

    #[test]
    fn simple_reflector() {
        let cipher = Cipher::from_string("YRUHQSLDPXNGOKMIEBFZCWVJAT");
        let reflector = Reflector::new(cipher);

        assert_eq!('Y', Cipher::to_char(reflector.sub(Cipher::from_char('A'))));
        assert_eq!('A', Cipher::to_char(reflector.sub(Cipher::from_char('Y'))));
    }

    #[test]
    #[should_panic(expected = "must be in pairs")]
    fn new_reflector_invalid_cipher() {
        let cipher = Cipher::from_string("RYUHQSLDPXNGOKMIEBFZCWVJAT");
        Reflector::new(cipher);
    }
}
