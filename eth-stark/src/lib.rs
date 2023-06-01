pub fn add(left: usize, right: usize) -> usize {
    left + right
}

extern "C" {
    fn doTheDooblyDoop(length: usize, blowup: usize);
}

pub fn run(length: usize, blowup: usize) {
    unsafe { doTheDooblyDoop(length, blowup); }
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
