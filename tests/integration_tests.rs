extern crate enigma;

use enigma::EnigmaBuilder;
use enigma::Alpha;
use enigma::Rotors;
use enigma::Reflectors;

#[test]
fn reciprocality() {
    use enigma::Alpha::*;

    let rotors = Rotors::new();
    let reflectors = Reflectors::new();
    let mut enigma = EnigmaBuilder::new()
        .left_rotor(rotors.i.with_ring_setting(A))
        .mid_rotor(rotors.ii.with_ring_setting(A))
        .right_rotor(rotors.iii.with_ring_setting(A))
        .window_positions([A, A, A])
        .reflector(reflectors.b)
        .build()
        .unwrap();

    let plaintext = Alpha::from_string("helloworld");
    let ciphertext = enigma.message(&plaintext);
    enigma.reset();
    let deciphered = enigma.message(&ciphertext);

    assert_eq!(
        Alpha::to_string(plaintext),
        Alpha::to_string(deciphered)
    )
}

#[test]
fn simple_cipher() {
    use enigma::Alpha::*;

    let rotors = Rotors::new();
    let reflectors = Reflectors::new();
    let mut enigma = EnigmaBuilder::new()
        .left_rotor(rotors.i.with_ring_setting(A))
        .mid_rotor(rotors.ii.with_ring_setting(A))
        .right_rotor(rotors.iii.with_ring_setting(A))
        .window_positions([A, A, A])
        .reflector(reflectors.b)
        .build()
        .unwrap();

    let plaintext = Alpha::from_string("AAAAA");
    let ciphertext = enigma.message(&plaintext);

    assert_eq!("BDZGO", Alpha::to_string(ciphertext))
}

#[test]
fn stepping() {
    use enigma::Alpha::*;

    let rotors = Rotors::new();
    let reflectors = Reflectors::new();
    let mut enigma = EnigmaBuilder::new()
        .left_rotor(rotors.i.with_ring_setting(A))
        .mid_rotor(rotors.ii.with_ring_setting(A))
        .right_rotor(rotors.iii.with_ring_setting(A))
        .window_positions([A, A, U])
        .reflector(reflectors.b)
        .build()
        .unwrap();

    enigma.press(A);
    assert_eq!([A,A,V], enigma.get_window_positions());

    enigma.press(A);
    assert_eq!([A,B,W], enigma.get_window_positions());

    enigma.press(A);
    assert_eq!([A,B,X], enigma.get_window_positions());
}

#[test]
fn double_stepping() {
    use enigma::Alpha::*;

    let rotors = Rotors::new();
    let reflectors = Reflectors::new();
    let mut enigma = EnigmaBuilder::new()
        .left_rotor(rotors.i.with_ring_setting(A))
        .mid_rotor(rotors.ii.with_ring_setting(A))
        .right_rotor(rotors.iii.with_ring_setting(A))
        .window_positions([A, D, U])
        .reflector(reflectors.b)
        .build()
        .unwrap();

    enigma.press(A);
    assert_eq!([A,D,V], enigma.get_window_positions());

    enigma.press(A);
    assert_eq!([A,E,W], enigma.get_window_positions());

    enigma.press(A);
    assert_eq!([B,F,X], enigma.get_window_positions());

    enigma.press(A);
    assert_eq!([B,F,Y], enigma.get_window_positions());
}
