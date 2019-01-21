struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("user1's email: {}", user1.email);
    println!("user1's sign in count: {}", user1.sign_in_count);
    user1.sign_in_count = 2;
    println!("user1's sign in count: {}", user1.sign_in_count);
}
