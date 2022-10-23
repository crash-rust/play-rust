const fn init_len() -> usize {
    5
}

pub fn main() {
    let arr = [0; init_len()];

    println!("arr len = {}", arr.len());
}
