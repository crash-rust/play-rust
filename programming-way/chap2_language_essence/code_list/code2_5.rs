pub fn main() {
    let place1 = "hello";
    let place2 = "hello".to_string();

    let _other1 = place1;
    println!("{:?}", place1);

    let _other2 = place2;
    // borrow of moved value 无法借用失去所有权的变量
    // println!("{:?}", place2);
}
