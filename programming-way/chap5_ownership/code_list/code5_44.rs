pub fn main() {
    let x = Box::new("hello");
    let _y = x;

    // x具有移动语义，当x仔赋值表达式右侧作为右值出现的时候，会将所有权直接移动给了_y，自身就失去了对堆数据的所有
    // println!("x is {x}");
}
