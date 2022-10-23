fn while_true(x: i32) -> i32 {
    // while true无限循环会错误，无限循环应该使用loop
    // while true {
    //     return x + 1;
    // }
    x
}

pub fn main() {
    while_true(32);
}
