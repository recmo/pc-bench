

#[no_mangle]
pub extern "C" fn hello_devworld() {
    for i in 0..10 {
        println!("Hello {i}");
    }
}

