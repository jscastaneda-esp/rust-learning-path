use super::authentication;

fn split_code_into_modules() {
    let user = authentication::User::new("jeremy", "super-secret");

    // println!("The username is: {}", user.username); // private field
    // println!("The password is: {}", user.password_hash); // private field
}
