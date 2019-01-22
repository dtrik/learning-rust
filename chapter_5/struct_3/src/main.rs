fn main() {
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    fn build_user(email: String, username: String) -> User {
        User {
            email,              //shorthand notation for email and username
            username,           //as function arg and struct field same name
            active: true,
            sign_in_count: 1,
        }
    }
}
