pub fn is_true() -> bool {
    true
}
fn true_maker() -> fn() -> bool {
    is_true
}

pub fn main() {
    assert_eq!(true_maker()(), true);
}
