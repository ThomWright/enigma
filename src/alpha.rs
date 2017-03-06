
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Alpha {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    J = 9,
    K = 10,
    L = 11,
    M = 12,
    N = 13,
    O = 14,
    P = 15,
    Q = 16,
    R = 17,
    S = 18,
    T = 19,
    U = 20,
    V = 21,
    W = 22,
    X = 23,
    Y = 24,
    Z = 25,
}

impl From<Alpha> for usize {
    fn from(letter: Alpha) -> usize {
        letter as usize
    }
}

impl From<Alpha> for char {
    fn from(letter: Alpha) -> char {
        use alpha::Alpha::*;
        match letter {
            A => 'A',
            B => 'B',
            C => 'C',
            D => 'D',
            E => 'E',
            F => 'F',
            G => 'G',
            H => 'H',
            I => 'I',
            J => 'J',
            K => 'K',
            L => 'L',
            M => 'M',
            N => 'N',
            O => 'O',
            P => 'P',
            Q => 'Q',
            R => 'R',
            S => 'S',
            T => 'T',
            U => 'U',
            V => 'V',
            W => 'W',
            X => 'X',
            Y => 'Y',
            Z => 'Z',
        }
    }
}

pub fn to_string(v: &[Alpha]) -> String {
    let mut cs = Vec::new();
    for c in v {
        cs.push(char::from(*c));
    }
    cs.into_iter().collect()
}

impl Alpha {
    pub fn from_string(s: &str) -> Vec<Alpha> {
        let mut v = Vec::new();
        for c in s.chars() {
            if let Ok(a) = Alpha::try_from_char(c) {
                v.push(a)
            }
        }
        v
    }

    pub fn try_from_char(c: char) -> Result<Alpha, &'static str> {
        use alpha::Alpha::*;
        use std::ascii::AsciiExt;
        match c.to_ascii_uppercase() {
            'A' => Ok(A),
            'B' => Ok(B),
            'C' => Ok(C),
            'D' => Ok(D),
            'E' => Ok(E),
            'F' => Ok(F),
            'G' => Ok(G),
            'H' => Ok(H),
            'I' => Ok(I),
            'J' => Ok(J),
            'K' => Ok(K),
            'L' => Ok(L),
            'M' => Ok(M),
            'N' => Ok(N),
            'O' => Ok(O),
            'P' => Ok(P),
            'Q' => Ok(Q),
            'R' => Ok(R),
            'S' => Ok(S),
            'T' => Ok(T),
            'U' => Ok(U),
            'V' => Ok(V),
            'W' => Ok(W),
            'X' => Ok(X),
            'Y' => Ok(Y),
            'Z' => Ok(Z),
            _ => Err("oh noes"),
        }
    }

    pub fn try_from_usize(n: usize) -> Result<Alpha, &'static str> {
        use alpha::Alpha::*;
        match n {
            0 => Ok(A),
            1 => Ok(B),
            2 => Ok(C),
            3 => Ok(D),
            4 => Ok(E),
            5 => Ok(F),
            6 => Ok(G),
            7 => Ok(H),
            8 => Ok(I),
            9 => Ok(J),
            10 => Ok(K),
            11 => Ok(L),
            12 => Ok(M),
            13 => Ok(N),
            14 => Ok(O),
            15 => Ok(P),
            16 => Ok(Q),
            17 => Ok(R),
            18 => Ok(S),
            19 => Ok(T),
            20 => Ok(U),
            21 => Ok(V),
            22 => Ok(W),
            23 => Ok(X),
            24 => Ok(Y),
            25 => Ok(Z),
            _ => Err("oh noes"),
        }
    }
}
