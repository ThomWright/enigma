use alpha::Alpha;
use cipher;
use cipher::Cipher;
use cipher::CipherChar;

/// A Rotor implements an arbitrary substitution cipher using wires connecting the 26 inputs to
/// the 26 outputs.
///
/// Each Rotor has an outer ring, and a core with the internal wiring.
///
/// The outer ring has each letter of the alphabet written on it. One of these letters will be
/// visible in the window of the Enigma machine. As a message is being typed, the rotors rotate or
/// 'step', changing which letter is visible. The visible letter is known as the 'window position'.
///
/// The Rotor's inner core can also be rotated with respect to the outer ring. This rotation is
/// known as the 'ring setting' or 'Ringstellung'.
///
/// These two rotations affect the position of the internal wiring.
///
/// There also is a notch set into the outer ring, which affects when the Rotors 'turnover'.
///
/// This Rotor implementation is immutable. It does not keep track of its own rotation within the
/// Enigma machine (the window position).
#[derive(Debug, Copy, Clone)]
pub struct Rotor {
    wiring: Cipher,
    inverse_wiring: Cipher,

    ring_setting: CipherChar,
    notch_position: CipherChar,
}

impl Rotor {
    fn new(wiring: Cipher, notch_position: CipherChar, ring_setting: CipherChar) -> Rotor {
        Rotor {
            wiring: wiring,
            inverse_wiring: Rotor::inverse_wiring(wiring),

            notch_position: notch_position,
            ring_setting: ring_setting,
        }
    }

    pub fn sub(&self, c: CipherChar) -> CipherChar {
        (self.wiring[(c + self.ring_setting) % 26] + 26 - self.ring_setting) % 26
    }

    pub fn inverse_sub(&self, c: CipherChar) -> CipherChar {
        (self.inverse_wiring[(c + self.ring_setting) % 26] + 26 - self.ring_setting) % 26
    }

    pub fn notch_engaged(&self, window_position: CipherChar) -> bool {
        (self.notch_position + 18) % 26 == window_position
    }

    fn inverse_wiring(wiring: Cipher) -> Cipher {
        let mut inverse = [0; 26];
        for (i, v) in wiring.into_iter().enumerate() {
            inverse[*v] = i;
        }
        inverse
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RotorFactory {
    wiring: Cipher,
    notch_position: CipherChar,
}

impl RotorFactory {
    pub fn with_ring_setting(&self, ring_setting: Alpha) -> Rotor {
        Rotor::new(self.wiring, self.notch_position, usize::from(ring_setting))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rotors {
    pub i: RotorFactory,
    pub ii: RotorFactory,
    pub iii: RotorFactory,
    pub iv: RotorFactory,
    pub v: RotorFactory,
}

impl Default for Rotors {
    fn default() -> Self {
        Rotors {
            i: RotorFactory {
                wiring: cipher::from_string("EKMFLGDQVZNTOWYHXUSPAIBRCJ"),
                notch_position: cipher::from_char('Y'),
            },
            ii: RotorFactory {
                wiring: cipher::from_string("AJDKSIRUXBLHWTMCQGZNPYFVOE"),
                notch_position: cipher::from_char('M'),
            },
            iii: RotorFactory {
                wiring: cipher::from_string("BDFHJLCPRTXVZNYEIWGAKMUSQO"),
                notch_position: cipher::from_char('D'),
            },
            iv: RotorFactory {
                wiring: cipher::from_string("ESOVPZJAYQUIRHXLNFTGKDCMWB"),
                notch_position: cipher::from_char('R'),
            },
            v: RotorFactory {
                wiring: cipher::from_string("VZBRGITYUPSDNHLXAWMJQOFECK"),
                notch_position: cipher::from_char('H'),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cipher as Cipher;
    use cipher::from_char as CC;

    #[test]
    fn noop_rotor() {
        let wiring = Cipher::from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let rotor = Rotor::new(wiring, CC('A'), CC('A'));

        assert_eq!(CC('A'), rotor.sub(CC('A')));
        assert_eq!(CC('A'), rotor.inverse_sub(CC('A')));
    }

    #[test]
    fn ring_position() {
        let wiring = Cipher::from_string("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let rotor = Rotor::new(wiring, CC('A'), CC('B'));

        assert_eq!(CC('A'), rotor.sub(CC('A')));
        assert_eq!(CC('A'), rotor.inverse_sub(CC('A')));
    }

    #[test]
    fn a_b_flip() {
        let wiring = Cipher::from_string("BACDEFGHIJKLMNOPQRSTUVWXYZ");
        let rotor = Rotor::new(wiring, CC('A'), CC('A'));

        assert_eq!(CC('B'), rotor.sub(CC('A')));
        assert_eq!(CC('A'), rotor.sub(CC('B')));
        assert_eq!(CC('Z'), rotor.sub(CC('Z')));
        assert_eq!(CC('B'), rotor.inverse_sub(CC('A')));
        assert_eq!(CC('A'), rotor.inverse_sub(CC('B')));
        assert_eq!(CC('Z'), rotor.inverse_sub(CC('Z')));
    }

    #[test]
    fn a_b_flip_with_ring_position() {
        let wiring = Cipher::from_string("BACDEFGHIJKLMNOPQRSTUVWXYZ");
        let rotor = Rotor::new(wiring, CC('A'), CC('B'));

        assert_eq!(CC('Z'), rotor.sub(CC('A')));
        assert_eq!(CC('B'), rotor.sub(CC('B')));
        assert_eq!(CC('A'), rotor.sub(CC('Z')));
        assert_eq!(CC('Z'), rotor.inverse_sub(CC('A')));
        assert_eq!(CC('B'), rotor.inverse_sub(CC('B')));
        assert_eq!(CC('A'), rotor.inverse_sub(CC('Z')));
    }
}
