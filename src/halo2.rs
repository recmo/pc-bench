use halo2_proofs::{
    arithmetic::{best_multiexp, Field},
    pasta::{
        group::{Curve, Group},
        pallas::{Affine, Point, Scalar},
    },
};
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::*;
use std::{mem::size_of, time::Instant};

pub fn rand_vec_point(size: usize) -> Vec<Affine> {
    const TABLE_SIZE: usize = 1_usize << 18;
    const CHUNK_SIZE: usize = 512;

    let now = Instant::now();
    println!(
        "Memory allocation ({} GB)",
        (size * size_of::<Affine>()) as f64 / 1.0e9
    );
    let mut result = vec![Affine::default(); size];

    // Generating random points using `Point::random` is very slow, so we
    // create a buffer of random points and then generate more
    // by taking the sum of two random points in the buffer.
    // In theory this allows for more efficient multi-exp, but
    // this requires some analysis on the base points that `best_multiexp`
    // doesn't do.

    // Create some random points in a table.
    println!("Creating a few real random points...");
    let random = {
        let mut rng = thread_rng();
        let mut points = vec![Point::default(); TABLE_SIZE];
        for point in points.iter_mut() {
            *point = Point::random(&mut rng);
        }
        let mut affine = vec![Affine::default(); TABLE_SIZE];
        Point::batch_normalize(&points, &mut affine);
        affine
    };

    // Fill the vector with the sum of two random points from the table.
    println!("Randomizing...");
    result.par_chunks_mut(CHUNK_SIZE).for_each_init(
        || (thread_rng(), vec![Point::default(); CHUNK_SIZE]),
        |(rng, buffer), chunk| {
            for point in buffer.iter_mut() {
                *point = random.choose(rng).unwrap() + random.choose(rng).unwrap();
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

pub fn run(max_exponent: usize) {
    let max_size = 1 << max_exponent;
    const DIVISIONS: usize = 8;

    println!("Preparing input points...");
    let points = rand_vec_point(max_size);
    println!("Preparing input scalars...");
    let scalars = rand_vec_scalar(max_size);

    println!("size,duration,throughput");

    for i in 10..=max_exponent {
        let size = 1_usize << i;
        let duration = bench_multi_exp(&points[..size], &scalars[..size]);
        let throughput = size as f64 / duration;
        println!("{size},{duration},{throughput}");

        if size >= max_size {
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
