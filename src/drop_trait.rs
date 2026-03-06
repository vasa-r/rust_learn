struct User {
    name: String,
}

impl Drop for User {
    fn drop(&mut self) {
        println!("Dropping value: {}", self.name);
    }
}

pub fn drop_trait() {
    let user = User {
        name: String::from("Vasanth"),
    };

    println!("User name is: {}", user.name);
}
