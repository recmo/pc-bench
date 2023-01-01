use halo2_proofs::{
    arithmetic::{best_multiexp, Field},
    pasta::pallas::{Affine, Point, Scalar},
    pasta::group::{Group, Curve},
};
use rand::{thread_rng, Fill, Rng};
use rayon::prelude::*;
use std::{
    mem::size_of,
    time::{Duration, Instant},
};

pub fn rand_vec_point(size: usize) -> Vec<Affine> {
    const CHUNK_SIZE: usize = 512;

    let now = Instant::now();
    println!(
        "Memory allocation ({} GB)",
        (size * size_of::<Affine>()) as f64 / 1.0e9
    );
    let mut result = vec![Affine::default(); size];
    println!("Randomizing...");

    result.par_chunks_mut(CHUNK_SIZE).for_each_init(
        || (thread_rng(), vec![Point::default(); CHUNK_SIZE]),
        |(rng, buffer), chunk| {
            for point in buffer.iter_mut() {
                *point = Point::random(&mut *rng);
            }
            Point::batch_normalize(&buffer, chunk);
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

fn bench_multi_exp(points: &[Affine], scalars: &[Scalar]) -> f64 {
    let mut count = 0;
    let mut duration = 0.0;
    // Halo2 takes care of parallelization.
    loop {
        count += 1;
        let now = Instant::now();

        let _ = best_multiexp(scalars, points);

        duration += now.elapsed().as_secs_f64();
        if duration > 5.0 {
            break;
        }
    }
    duration / count as f64
}

fn main() {
    const MAX_EXPONENT: u32 = 28;
    const MAX_SIZE: usize = 1 << MAX_EXPONENT;
    const DIVISIONS: usize = 8;

    println!("Preparing input points...");
    let points = rand_vec_point(MAX_SIZE);
    println!("Preparing input scalars...");
    let scalars = rand_vec_scalar(MAX_SIZE);

    println!("size,duration,throughput");

    for i in 10..=MAX_EXPONENT {
        let size = 1_usize << i;
        let duration = bench_multi_exp(&points[..size], &scalars[..size]);
        let throughput = size as f64 / duration;
        println!("{size},{duration},{throughput}");

        if size >= MAX_SIZE {
            break;
        }
        for i in 1..DIVISIONS {
            let fraction = 2.0_f64.powf(i as f64 / DIVISIONS as f64);
            let size = (size as f64 * fraction).round() as usize;
            let duration = bench_multi_exp(&points[..size], &scalars[..size]);
            let throughput = size as f64 / duration;
            println!("{size},{duration},{throughput}");
        }
    }
}
