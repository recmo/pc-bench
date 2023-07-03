use clap::ValueEnum;

pub mod ark;
pub mod blst;
pub mod halo2;
pub mod plonky2;
pub mod pse;
pub mod winter;
pub mod risc0;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, ValueEnum)]
pub enum Algorithm {
    Blst,
    Halo2,
    PSE,
    Plonky2_Keccak,
    Plonky2_Keccak_Batch16,
    Plonky2_Keccak_Batch256,
    Plonky2_Poseidon,
    Ark,
    Winter,
    Risc0,
}

pub fn run(algorithm: Algorithm, max_exponent: usize) {
    use Algorithm::*;
    match algorithm {
        Blst => blst::run(max_exponent),
        Halo2 => halo2::run(max_exponent),
        PSE => pse::run(max_exponent),
        Plonky2_Keccak => plonky2::run(max_exponent, false, 1),
        Plonky2_Keccak_Batch16 => plonky2::run(max_exponent, false, 16),
        Plonky2_Keccak_Batch256 => plonky2::run(max_exponent, false, 256),
        Plonky2_Poseidon => plonky2::run(max_exponent, true, 1),
        Ark => ark::run(max_exponent),
        Winter => winter::run(max_exponent),
        Risc0 => risc0::run(max_exponent),
    }
}
