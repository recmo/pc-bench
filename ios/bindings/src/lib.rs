use pc_bench;
use std::time::Instant;

#[no_mangle]
pub extern "C" fn run() {
    let start = Instant::now();
    pc_bench::run(pc_bench::Algorithm::Stark, 16);
    let duration = start.elapsed().as_secs_f64();
    println!("Done in {duration}s!");
}

