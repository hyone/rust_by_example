fn main() {
    let u: &[u8] = &[49, 50, 51];
    let s: &str = "123";

    assert!(u == s.as_bytes());

    unsafe {
        assert!(u == std::mem::transmute::<&str, &[u8]>(s));
    }
}
