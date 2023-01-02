use pc_bench;

#[no_mangle]
pub extern "C" fn run() {
    pc_bench::halo2::run();
    println!("Done!");
}

