// 元组结构体相当于是有了名字的元组

// 定义元组结构体
#[derive(Debug)]
struct Color(i32, i32, i32);

// 创建元组结构体
fn create_color(r: i32, g: i32, b: i32) -> Color {
    Color(r, g, b)
}

// 获取元组结构体的字段
fn get_red_channel(color: &Color) -> i32 {
    color.0
}

pub fn use_tuple_struct() {
    let white = create_color(255, 255, 255);
    println!("white = {:?}", white);

    let red_channel = get_red_channel(&white);
    println!("red_channel = {}", red_channel);
}
