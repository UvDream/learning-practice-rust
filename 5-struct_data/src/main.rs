// 打印结构体
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        name: String::from("John"),
        email: String::from("uvdream@13.com"),
        sign_in_count: 1,
        active: false,
    };
    user1.name = String::from("张三");
    println!("{:#?}", user1);
    let new_user = build_user(user1.email, user1.name);
    println!("{:#?}", new_user);
    // 3.结构体更新语法
    let user2 = User {
        email: String::from("another@example.com"),
        name: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // 简介写法
    let user3 = User {
        email: String::from("another@example.com"),
        name: String::from("anotherusername567"),
        ..user1
    };
    println!("更新结构体");
    println!("{:#?}--{:#?}", user2, user3)
}
// 1.复用值
fn build_user(email: String, name: String) -> User {
    // User {
    //     email: email,
    //     name: name,
    //     active: true,
    //     sign_in_count: 2,
    // }
    // 2.简介写法
    User {
        email,
        name,
        active: true,
        sign_in_count: 2,
    }
}
