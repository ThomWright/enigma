# Enigma

M3 Enigma machine

## TODO

- [x] Rotor
    - [X] Ring setting
    - [X] Notch position
    - [ ] 5 Rotor types
- [x] Reflector
    - [ ] 2 Reflector types
- [ ] Machine
    - [ ] Rotor Window positions
    - [ ] Stepping
    - [ ] Turnover
    - [ ] Double stepping
- [ ] Plugboard

## Settings

http://www.codesandciphers.org.uk/enigma/rotorspec.htm

### Rotors

#### Wiring

- Rotor I    | EKMFLGDQVZNTOWYHXUSPAIBRCJ
- Rotor II   | AJDKSIRUXBLHWTMCQGZNPYFVOE
- Rotor III  | BDFHJLCPRTXVZNYEIWGAKMUSQO
- Rotor IV   | ESOVPZJAYQUIRHXLNFTGKDCMWB
- Rotor V    | VZBRGITYUPSDNHLXAWMJQOFECK
- Rotor VI   | FVPJIAOYEDRZXWGCTKUQSBNMHL
- Rotor VII  | NZJHGRCXMYSWBOUFAIVLPEKQDT
- Rotor VIII | FKQHTLXOCBJSPDZRAMEWNIUYGV

#### Notches

| Rotor | Notch | Window | next left rotor steps when rotor steps from/to |
|-------|-------|--------|------------------------------------------------|
| I     | Y     | Q      | Q -> R                                         |
| II    | M     | E      | E -> F                                         |
| III   | D     | V      | V -> W                                         |
| IV    | R     | J      | J -> K                                         |
| V     | H     | Z      | Z -> A                                         |

Notch + 18 = Window

### Reflectors

- Reflector B	(AY) (BR) (CU) (DH) (EQ) (FS) (GL) (IP) (JX) (KN) (MO) (TZ) (VW) - YRUHQSLDPXNGOKMIEBFZCWVJAT
- Reflector C	(AF) (BV) (CP) (DJ) (EI) (GO) (HY) (KR) (LZ) (MX) (NW) (TQ) (SU) - FVPJIAOYEDRZXWGCTKUQSBNMHL


---
http://www.mlb.co.jp/linux/science/genigma/enigma-referat/node4.html

Example

This a ciphered text:

215 AAA FRA "ABIRUXKP" PCDAONONEBCJBOGLYMEEYGSHRYUBUJHMJOQZLEX

The first line is the setup.

215 rotor order (Walzenlage) means left wheel number 2, middle wheel number one and right wheel number 5.

AAA is the corresponding ring setting (Ringstellung).

FRA is the corresponding starting position (Grundstellung).

AB IR UX KP Plugboard connections (Steckerverbindungen).

And this is the enciphered plain text.

ANBULMEGRAZGOESTINGSTRENGGEHEIMEMELDUNG
