use std::path::Path;
use std::env;

fn main() {
    let libdir = "vendor-eth-stark/build/target/lib";
    eprintln!("current dir {:?}", env::current_dir());
    println!("cargo:rerun-if-changed=vendor-eth-stark/CMakeLists.txt");

    // println!("cargo:rerun-if-changed=../../ethSTARK/CMakeLists.txt");
    let contents = std::fs::read_dir(libdir).unwrap();
    for entry in contents {
        println!("{:?}", entry);
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let libname = name.strip_prefix("lib").unwrap().strip_suffix(".a").unwrap();
        println!("cargo:rustc-link-lib=static={libname}");
    }
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=static=glog");
    println!("cargo:rustc-link-lib=static=gflags");

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}", Path::new(&dir).join(libdir).display());
    println!("cargo:rustc-link-search={}", Path::new(&dir).join("vendor-eth-stark/vendor/lib").display());

}