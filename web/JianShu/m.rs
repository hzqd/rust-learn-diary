//rust--如何结构体中定义&str数据类型?
// 答案是 声明生命周期

#[derive(Debug)]
struct User<'a> {               // 声明泛型生命周期参数
    username: &'a str,         // 声明泛型引用
    email: &'a str,            // 声明泛型引用
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
    println!("{:?}", user1)
}