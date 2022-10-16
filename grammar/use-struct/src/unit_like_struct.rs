// 无任何字段的结构体，目的常常就是用来实现trait而已
#[derive(Debug)]
struct AlwaysEqual;

pub fn use_unit_like_struct() {
    let subject = AlwaysEqual;

    println!("subject = {:?}", subject);
}
