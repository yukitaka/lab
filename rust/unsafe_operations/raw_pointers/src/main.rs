fn main() {
    let raw_p: *const u32 = &10;
    unsafe {
        assert_eq!(*raw_p, 10);
    }
}
