use log::Level;
use rand::{thread_rng, Fill, Rng};
use rayon::prelude::*;
use std::{
    mem::size_of,
    time::{Duration, Instant},
};
use risc0_zkp::hal::Hal;
use risc0_zkp::{self,core::fp::Fp as F, prove::poly_group::PolyGroup, hal::cpu::CpuHal};

pub fn rand_vec(size: usize) -> Vec<F> {
    let now = Instant::now();
    println!(
        "Memory allocation ({} GB)",
        (size * size_of::<F>()) as f64 / 1.0e9
    );
    let mut result = vec![F::default(); size];
    println!("Randomizing...");
    result.par_chunks_mut(1024).for_each_init(
        || thread_rng(),
        |rng, chunk| {
            for point in chunk {
                *point = F::new(rng.gen());
            }
        },
    );
    println!("Random generation took: {:?}", now.elapsed());
    result
}

fn bench(input: &[F]) -> f64 {
    let mut count = 0;
    let mut duration = 0.0;

    let hal = CpuHal {};
    let buffer = hal.copy_from(input);

    loop {
        count += 1;
        let now = Instant::now();

        let _ = PolyGroup::new(&hal, &buffer, 1, input.len());

        duration += now.elapsed().as_secs_f64();
        if duration > 5.0 {
            break;
        }
    }
    duration / count as f64
}

pub fn run(max_exponent: usize) {
    let max_size = 1 << max_exponent;
    println!("Preparing input...");
    let input = rand_vec(max_size);

    println!("size,duration,throughput");

    for i in 10..=max_exponent {
        let size = 1_usize << i;
        let duration = bench(&input[..size]);
        let throughput = size as f64 / duration;
        println!("{size},{duration},{throughput}");
    }
}
