#[derive(Debug)]
struct User {
    #[allow(unused)]
    active: bool,
    username: String,
    email: String,
    #[allow(unused)]
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername"),
    );

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait
    // println!("User1: {}, {}", user1.email, user1.username);
    println!("User2: {:?}", user2);
    println!("User2: {:#?}", user2);
    dbg!(&user2);
}
