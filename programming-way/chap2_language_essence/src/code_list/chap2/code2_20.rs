pub fn main() {
    let number = 42;
    match number {
        0 => println!("Origin"),
        1 | 2 | 3 => println!("All"),
        5 | 7 | 13 => println!("Bad Luck"),
        n @ 42 => println!("target = {}", n),
        _ => println!("Common"),
    }
}
