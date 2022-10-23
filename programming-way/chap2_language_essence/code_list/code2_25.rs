pub fn main() {
    #[allow(unused)]
    let num = 42u32;
    #[allow(unused)]
    let num: u32 = 42;
    #[allow(unused)]
    let num = 0x2A;
    #[allow(unused)]
    let num = 0o106;
    #[allow(unused)]
    let num = 0b1101_1011;

    assert_eq!(b'*', 42u8);
    assert_eq!(b'\'', 39u8);

    #[allow(unused)]
    let num = 3.1415926f64;

    assert_eq!(-3.14, -3.14f64);
    assert_eq!(2., 2.0f64);
    assert_eq!(2e4, 20000f64);

    println!("{:?}", std::f32::INFINITY);
    println!("{:?}", std::f32::NEG_INFINITY);
    println!("{:?}", std::f32::NAN);
    println!("{:?}", std::f32::MIN);
    println!("{:?}", std::f32::MAX);
}
