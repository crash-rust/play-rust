/// 值表达式不能出现在位置上下文中

pub fn temp() -> i32 {
    return 1;
}

pub fn main() {
    let x = &temp();
    println!("x = {}", x);
    // temp()是值表达式，不能放在赋值语句左侧，赋值语句左侧的操作数是位置表达式
    // temp() = *x;
}
