struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn from_email(&mut self) {
        // extract name from an email
        if let Some(pos) = self.email.find('@') {
            self.username = self.email[..pos].to_string();
        }
    }
    fn update_uri(&mut self, new_uri: String) {
        self.uri = new_uri;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("wojtekpudelko@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.from_email();
    println!("Username extracted from email is: {}", new_user.username);
    new_user.update_uri(String::from("https://wojtekpudelko.com"));
    println!("Updated URI is: {}", new_user.uri);
}
