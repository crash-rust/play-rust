#[allow(dead_code)]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub fn main() {
    // 传入与函数契约不同的参数类型，会导致函数报错！
    // sum(1u32, 2u32);
}
