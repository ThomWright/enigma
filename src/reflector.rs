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
    pub c: Reflector,
}

impl Reflectors {
    pub fn new() -> Reflectors {
        Reflectors {
            b: Reflector {
                cipher: cipher::from_string("YRUHQSLDPXNGOKMIEBFZCWVJAT")
            },
            c: Reflector {
                cipher: cipher::from_string("FVPJIAOYEDRZXWGCTKUQSBNMHL")
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cipher as Cipher;

    #[test]
    fn simple_reflector() {
        let cipher = Cipher::from_string("YRUHQSLDPXNGOKMIEBFZCWVJAT");
        let reflector = Reflector::new(cipher);

        assert_eq!(24, reflector.sub(0));
        assert_eq!(0, reflector.sub(24));
    }

    #[test]
    #[should_panic(expected = "must be in pairs")]
    fn new_reflector_invalid_cipher() {
        let cipher = Cipher::from_string("RYUHQSLDPXNGOKMIEBFZCWVJAT");
        Reflector::new(cipher);
    }
}
