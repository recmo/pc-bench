use clap::ValueEnum;

pub mod ark;
pub mod blst;
pub mod halo2;
pub mod plonky2;
pub mod pse;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Algorithm {
    Blst,
    Halo2,
    PSE,
    Plonky2_Keccak,
    Plonky2_Poseidon,
    Ark,
}

pub fn run(algorithm: Algorithm, max_exponent: usize) {
    use Algorithm::*;
    match algorithm {
        Blst => blst::run(max_exponent),
        Halo2 => halo2::run(max_exponent),
        PSE => pse::run(max_exponent),
        Plonky2_Keccak => plonky2::run(max_exponent, false),
        Plonky2_Poseidon => plonky2::run(max_exponent, true),
        Ark => ark::run(max_exponent),
    }
}
