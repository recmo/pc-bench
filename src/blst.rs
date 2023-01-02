use blstrs::{G1Affine, G1Projective, Scalar};
use ff::Field;
use group::Group;
use rand::{thread_rng, Fill, Rng};
use rayon::prelude::*;
use std::{
    mem::size_of,
    time::{Duration, Instant},
};

pub fn rand_vec_g1(size: usize) -> Vec<G1Projective> {
    let now = Instant::now();
    println!(
        "Memory allocation ({} GB)",
        (size * size_of::<G1Projective>()) as f64 / 1.0e9
    );
    let mut result = vec![G1Projective::generator(); size];
    println!("Randomizing...");
    result.par_chunks_mut(1000).for_each_init(
        || thread_rng(),
        |rng, chunk| {
            for point in chunk {
                *point = G1Projective::random(&mut *rng);
            }
        },
    );
    println!("Random generation took: {:?}", now.elapsed());
    result
}

pub fn rand_vec_scalar(size: usize) -> Vec<Scalar> {
    let now = Instant::now();
    println!(
        "Memory allocation ({} GB)",
        (size * size_of::<Scalar>()) as f64 / 1.0e9
    );
    let mut result = vec![Scalar::zero(); size];
    println!("Randomizing...");
    result.par_chunks_mut(1024).for_each_init(
        || thread_rng(),
        |rng, chunk| {
            for point in chunk {
                *point = Scalar::random(&mut *rng);
            }
        },
    );
    println!("Random generation took: {:?}", now.elapsed());
    result
}

fn bench_g1_multi_exp(points: &[G1Projective], scalars: &[Scalar]) -> f64 {
    let mut count = 0;
    let mut duration = 0.0;
    // BLST takes care of parallelization.
    loop {
        count += 1;
        let now = Instant::now();
        let _ = G1Projective::multi_exp(points, scalars);
        duration += now.elapsed().as_secs_f64();
        if duration > 5.0 {
            break;
        }
    }
    duration / count as f64
}

pub fn run() {
    const MAX_EXPONENT: u32 = 15;
    const MAX_SIZE: usize = 1 << MAX_EXPONENT;
    const DIVISIONS: usize = 8;

    println!("Preparing input points...");
    let points = rand_vec_g1(MAX_SIZE);
    println!("Preparing input scalars...");
    let scalars = rand_vec_scalar(MAX_SIZE);

    println!("size,duration,throughput");

    for i in 10..=MAX_EXPONENT {
        let size = 1_usize << i;
        let duration = bench_g1_multi_exp(&points[..size], &scalars[..size]);
        let throughput = size as f64 / duration;
        println!("{size},{duration},{throughput}");

        if size >= MAX_SIZE {
            break;
        }
        for i in 1..DIVISIONS {
            let fraction = 2.0_f64.powf(i as f64 / DIVISIONS as f64);
            let size = (size as f64 * fraction).round() as usize;
            let duration = bench_g1_multi_exp(&points[..size], &scalars[..size]);
            let throughput = size as f64 / duration;
            println!("{size},{duration},{throughput}");
        }
    }
}
