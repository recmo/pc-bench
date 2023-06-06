use std::path::Path;
use std::env;

fn main() {
    let cargodir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let libdir = "vendor-eth-stark/build/target/lib";
    let libdir = Path::new(&cargodir).join(libdir);

    eprintln!("current dir {:?}", env::current_dir());
    eprintln!("reading dir {:?}", libdir);
    println!("cargo:rerun-if-changed=vendor-eth-stark/CMakeLists.txt");

    // println!("cargo:rerun-if-changed=../../ethSTARK/CMakeLists.txt");
    let contents = std::fs::read_dir(&libdir).unwrap();
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
    println!("cargo:rustc-link-search={}", libdir.display());
    println!("cargo:rustc-link-search={}", Path::new(&cargodir).join("vendor-eth-stark/vendor/lib").display());

}