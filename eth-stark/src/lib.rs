use std::{time::Instant, ffi::c_void};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

extern "C" {
    fn free_obj(input: *mut c_void);
    fn new_input(trace_length: usize) -> *mut c_void;
    fn new_prover_factory() -> *mut c_void;
    fn run_bench(input: *mut c_void, prover_factory: *mut c_void, length: usize, blowup: usize);
}

pub fn bench(size: usize, blowup: usize) -> f64 {
    let mut count = 0;
    let mut duration = 0.0;

    let input = unsafe { new_input(size) };
    let prover_factory = unsafe { new_prover_factory() };

    loop {
        count += 1;
        let now = Instant::now();

        unsafe { run_bench(input, prover_factory, size, blowup) };

        duration += now.elapsed().as_secs_f64();
        if duration > 5.0 {
            break;
        }
    }

    //unsafe { free_obj(prover_factory) };
    unsafe { free_obj(input) };

    duration / count as f64
}

pub fn run(max_exponent: usize, blowup: usize) {
    println!("size,duration,throughput");
    for i in 10..=max_exponent {
        let size = 1_usize << i;
        let duration = bench(size, blowup);
        let throughput = size as f64 / duration;
        println!("{size},{duration},{throughput}");

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        run(10, 2)
    }
}
