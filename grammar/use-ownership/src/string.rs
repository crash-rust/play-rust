pub fn use_string() {
    let mut s = String::from("hello");
    s.push_str(" world!");
    // 使用push_str可以往字符串追加slice切片，因为字符串是可变的
    println!("s = {}", s);

    let s1 = String::from("hello");
    let s2 = s1;
    // s1在上面的时候就将字符串的所有权转移给了s2，因此这里一定会编译出错！
    // println!("s1 {}", s1);
    println!("s2 {}", s2);

    let s3 = String::from("hello");
    let s3_clone = s3.clone();
    // 使用clone的方式，s3就不会转移数据所有权，而是在内存当中开辟一块新的内存创建一份数据副本
    println!("s3 = {}", s3);
    println!("s3_clone = {}", s3_clone);

    let x = 5;
    let y = x;
    // 如果一个类型实现了copy trait，那么这个值所绑定的变量在拷贝的时候不会转移这个值的所有权
    // 在这里就是x拷贝给y，但是x不会将5的所有权转移给y，而是直接在stack上拷贝一份新的值5，让y绑定新的5
    // 实现copy trait的类型有:
    // 整数类型 | 布尔类型 | 浮点数类型 | 字符类型 | 成员中全是实现了copy trait的元组类型
    println!("x = {}", x);
    println!("y = {}", y);

    let s = String::from("hello");
    takes_ownership(s);
    // 当使用了takes_ownership函数时，需要往里头传入String类型的值
    // 此时s传入了takes_ownership，失去了字符串的所有权，于是下面尝试再次使用s的时候会报错，因为s失去了字符串的拥有权利
    // 跨函数传递拥有数据所有权的变量时，数据所有权会被传递到函数里头去，此时外边的变量就失去了这份数据的掌控权力
    // println!("s = {}", s);
}

fn takes_ownership(some_string: String) {
    println!("some_string = {}", some_string);
}
