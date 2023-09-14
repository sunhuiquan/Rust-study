#![allow(unused)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    // --------------------------------------------------------

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
        ..user1
    };

    println!("{}", user1.active);
    // 下面这行会报错
    // println!("{:?}", user1); error: user1.username borrowed here after partial move
    println!("{:?}", user2);
}
