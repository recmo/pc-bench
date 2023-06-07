use std::time::Instant;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

extern "C" {
    fn runBenchmark(length: usize, blowup: usize);
}

pub fn bench(size: usize, blowup: usize) -> f64 {
    let mut count = 0;
    let mut duration = 0.0;

    loop {
        count += 1;
        let now = Instant::now();

        unsafe { runBenchmark(size, blowup); }

        duration += now.elapsed().as_secs_f64();
        if duration > 5.0 {
            break;
        }
    }
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
        unsafe {
            doTheDooblyDoop(16, 2);
        }
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
