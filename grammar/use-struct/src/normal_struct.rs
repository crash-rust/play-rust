// 通过struct关键字定义结构体
// 结构体的成员叫做字段
#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 使用结构体
pub fn use_struct() {
    // 将结构体实例标记为mut，才可以创建可变引用&mut user
    let mut user = create_user(
        true,
        String::from("jaylenchan"),
        String::from("jaylen.work@hotmail.com"),
        1,
    );
    println!("user = {:?}", user);

    let email = get_user_email(&user);
    println!("email = {}", email);

    // 只有user实例被标记成为mut，才可以创建&mut user，用来修改user实例成员
    alter_user_email(&mut user);
    println!("after alter email = {}", user.email);
}

// 创建结构体
fn create_user(active: bool, username: String, email: String, sign_in_count: u64) -> User {
    // 字段名与传入变量同名，可以使用简写语法创建结构体
    User {
        active,
        username,
        email,
        sign_in_count,
    }
}

// 获取结构体的值
fn get_user_email(user: &User) -> String {
    // 这里使用clone的原因是因为email这个字段是个字符串，如果不clone会将所有权从user.email身上转移到新变量身上
    let email = user.email.clone();

    email
}

// 修改结构体的值
fn alter_user_email(user: &mut User) {
    user.email = "jaylen.work@gmail.com".to_string();
}
