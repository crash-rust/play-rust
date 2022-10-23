fn while_true(x: i32) -> i32 {
    #[allow(while_true)]
    while true {
        return x + 1;
    }

    x
}

pub fn main() {
    while_true(32);
}
