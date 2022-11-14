pub fn main() {
    let orig = Box::new(5);
    println!("orig is {}", *orig);

    let _stolen = orig;

    // 由于orig在上边将数据的所有权交给了_stolen，所以orig失去了数据的所有权，因此没法通过解引用来获取数据的访问
    // println!("orig is {}", *orig);
}
