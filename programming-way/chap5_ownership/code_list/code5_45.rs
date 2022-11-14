pub fn main() {
    let a = Box::new("hello");
    let b = Box::new("rust".to_string());

    let _c = *a;
    let _d = *b;

    // a还能够使用是因为a装的数据是字符串切片可以进行按位复制
    println!("a is {a}");
    //  b不能使用的原因是因为b装的数据是字符串，本身就具有移动语义，没法进行按位复制
    // println!("b is {b}");
}
