#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unused_extern_crates,
        unused_import_braces, unused_qualifications)]

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

pub use self::alpha::Alpha;
pub use self::alpha::to_string as alphas_to_string;
pub use self::enigma::EnigmaBuilder;
pub use self::rotor::Rotors;
pub use self::reflector::Reflectors;

mod enigma;

mod rotor;
mod reflector;

mod cipher;

mod alpha;
