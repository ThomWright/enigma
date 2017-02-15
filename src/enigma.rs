use rotor::Rotor;
use reflector::Reflector;

use alpha::Alpha;
use cipher::CipherChar;

pub struct Enigma {
    rotors: [RR; 3], // from left to right
    reflector: Reflector,

    original_window_positions: [CipherChar; 3],
}

/// This struct tracks the rotation of a Rotor within the Enigma machine.
struct RR {
    pub rotor: Rotor,
    pub window_position: CipherChar,
}

impl Enigma {
    pub fn message(&mut self, cipher_text: &Vec<Alpha>) -> Vec<Alpha> {
        let mut v = Vec::new();
        for l in cipher_text {
            v.push(self.press(*l));
        }
        v
    }

    pub fn press(&mut self, letter: Alpha) -> Alpha {
        self.step_and_turnover();
        let result = self.encipher(usize::from(letter));
        Alpha::try_from_usize(result).unwrap()
    }

    pub fn reset(&mut self) {
        for (i, pos) in self.original_window_positions.iter().enumerate() {
            self.rotors[i].window_position = *pos;
        }
    }

    pub fn get_window_positions(&self) -> [Alpha; 3] {
        let mut positions = [Alpha::A; 3];
        for (i, rr) in self.rotors.iter().enumerate() {
            positions[i] = Alpha::try_from_usize(rr.window_position).unwrap();
        }
        positions
    }

    fn encipher(&self, letter: CipherChar) -> CipherChar {
        let mut cipher_letter = letter;
        for rr in self.rotors.iter().rev() {
            let cl = cipher_letter;
            let wp = rr.window_position;
            let sub = |l| rr.rotor.sub(l);
            cipher_letter = (sub((cl + wp) % 26) + 26 - wp) % 26;
        }
        cipher_letter = self.reflector.sub(cipher_letter);
        for rr in self.rotors.iter() {
            let cl = cipher_letter;
            let wp = rr.window_position;
            let sub = |l| rr.rotor.inverse_sub(l);
            cipher_letter = (sub((cl + wp) % 26) + 26 - wp) % 26;
        }
        cipher_letter
    }

    fn step_and_turnover(&mut self) {
        let mut to_step = [false, false, true];
        for (i, rr) in self.rotors.iter().enumerate() {
            let engaged = rr.rotor.notch_engaged(rr.window_position);
            if engaged&& i != 0 {
                to_step[i] = true;
                to_step[i-1] = true;
            }
        }
        for (i, &should_step) in to_step.iter().enumerate() {
            if should_step {
                self.step(i);
            }
        }
    }

    fn step(&mut self, rotor_index: usize) {
        self.rotors[rotor_index].window_position = (self.rotors[rotor_index].window_position + 1) % 26;
    }
}

pub struct EnigmaBuilder {
    left_rotor: Option<Rotor>,
    mid_rotor: Option<Rotor>,
    right_rotor: Option<Rotor>,

    window_positions: Option<[CipherChar; 3]>,

    reflector: Option<Reflector>,
}

impl EnigmaBuilder {
    pub fn new() -> EnigmaBuilder {
        EnigmaBuilder{
            left_rotor: None,
            mid_rotor: None,
            right_rotor: None,
            window_positions: None,
            reflector: None,
        }
    }

    pub fn left_rotor(mut self, rotor: Rotor) -> EnigmaBuilder {
        self.left_rotor = Some(rotor);
        self
    }

    pub fn mid_rotor(mut self, rotor: Rotor) -> EnigmaBuilder {
        self.mid_rotor = Some(rotor);
        self
    }

    pub fn right_rotor(mut self, rotor: Rotor) -> EnigmaBuilder {
        self.right_rotor = Some(rotor);
        self
    }

    pub fn reflector(mut self, reflector: Reflector) -> EnigmaBuilder {
        self.reflector = Some(reflector);
        self
    }

    pub fn window_positions(mut self, positions: [Alpha; 3]) -> EnigmaBuilder {
        let mut window_positions: [CipherChar; 3] = [0; 3];
        for (i, &v) in positions.iter().enumerate() {
            window_positions[i] = usize::from(v);
        }
        self.window_positions = Some(window_positions);
        self
    }

    pub fn build(self) -> Result<Enigma, &'static str> {
        let window_positions = match self.window_positions {
            Some(r) => r,
            None => return Err("Must supply initial rotor positions"),
        };
        let left_rotor = match self.left_rotor {
            Some(r) => RR {
                rotor: r,
                window_position: window_positions[0]
            },
            None => return Err("Must supply left rotor"),
        };
        let mid_rotor = match self.mid_rotor {
            Some(r) => RR {
                rotor: r,
                window_position: window_positions[1]
            },
            None => return Err("Must supply mid rotor"),
        };
        let right_rotor = match self.right_rotor {
            Some(r) => RR {
                rotor: r,
                window_position: window_positions[2]
            },
            None => return Err("Must supply right rotor"),
        };
        let reflector = match self.reflector {
            Some(r) => r,
            None => return Err("Must supply reflector"),
        };
        Ok(Enigma {
            rotors: [left_rotor, mid_rotor, right_rotor],
            reflector: reflector,
            original_window_positions: window_positions,
        })
    }
}
