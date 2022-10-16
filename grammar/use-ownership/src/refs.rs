pub fn use_refs() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("len = {}", len);
    // 可以使用s1的引用来获取数据，而不是直接将s1传递到calculate_length中去
    // 使用引用可以避免s1失去数据所有权
    println!("s1 = {}", s1);

    let mut s2 = String::from("hello");
    let ref_s2 = &mut s2;
    // 不能在同一时间内创建两个可变引用，准确点说是在第一个可变引用被创建后，如果还没使用完成，在此期间就不能继续创建第二个可变引用
    // 即在一个可变引用被创建到使用期间，只要有一个地方还在使用这个可变引用，那么在此期间的作用范围都不能创建第二个可变引用）
    // 验证：可以尝试将ref_s2_copy的声明放到println!("ref_s2 = {}", ref_s2);之后，会看到报错就消失了
    // 这个限制的好处是 Rust 可以在编译时就避免数据竞争
    // let ref_s2_copy = &mut s2;
    append_str(ref_s2);
    // 引用的传递不会失去所有权
    println!("ref_s2 = {}", ref_s2);
    // 可以通过传递可变引用来改变字符串的值而不用直接传递字符串所有权到函数里头
    println!("new_s2 = {}", s2);

    let mut s = String::from("hello");
    // 允许同时具有多个不可变引用
    let s_ref = &s;
    let s_ref1 = &s;
    // 在不可变引用被使用完之前没法同时创建出可变引用，以下会报错
    // let s_mut_ref = &mut s;

    println!("s_ref = {}", s_ref);
    println!("s_ref1 = {}", s_ref1);
    // 但是当不可变引用被引用完成之后，就可以创建可变引用了，以下代码不会报错
    // 注意：一个引用的作用域为 - 从声明的地方开始，一直持续到最后一次使用为止
    let s_mut_ref = &mut s;
    println!("s_mut_ref = {}", s_mut_ref);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn append_str(s: &mut String) {
    s.push_str(" world!");
}

// 无法在函数作用域中声明的变量的引用返回，因为会造成悬垂引用
// 以下函数中创建了一个字符串s，但是试图将这个字符串的引用返回
// 由于字符串s的出了函数就会被drop释放，而我们试图将一个drop释放的值的引用返回让外边继续使用，就会让编译报错
// fn dangle_error<'a>() -> &'a String {
//   let s = String::from("hello");
//   &s
// }
