use std::env;

fn main() {
    let libdir = "../ethSTARK/build/target/lib";
    eprintln!("{:?}", env::current_dir());
    println!("cargo:rerun-if-changed=../ethSTARK/CMakeLists.txt");
    // println!("cargo:rerun-if-changed=../../ethSTARK/CMakeLists.txt");
    let contents = std::fs::read_dir(format!("../{libdir}")).unwrap();
    for entry in contents {
        println!("{:?}", entry);
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap();
        let libname = name.strip_prefix("lib").unwrap().strip_suffix(".a").unwrap();
        println!("cargo:rustc-link-lib={libname}");
    }
    println!("cargo:rustc-link-lib=c++");
    println!("cargo:rustc-link-lib=glog");
    println!("cargo:rustc-link-lib=gflags");
    println!("cargo:rustc-link-search=native={libdir}");
}