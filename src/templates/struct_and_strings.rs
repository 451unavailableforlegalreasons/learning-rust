
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,

}

fn build_user(username: String, email:String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

impl User {
    fn change_username(&mut self, new_username: String) {
        self.username = new_username;
    }
}

fn function (name: String) -> String {
    println!("Hello {}", name);
    return format!("Hello {}", name).to_string();
}

fn main() {
    let name: String = "paul".to_string();
    println!("{}", function(name));

    let mut user1: User = User {
        username: String::from("451"),
        email: String::from("p4ul.claret@gmail.com"),
        sign_in_count: 1,
        active: true,
    };
    let name: String = user1.username;
    user1.username = String::from("451-unavailable-for-legal-reasons");
    println!("{} {}", name, user1.username);
    let user2 = build_user("paulclrt".to_string(), "p4ul.claret@gmail.com".to_string());
    let mut user3 = User {
        username: String::from("paulclaret"),
        email: String::from("p4ul.claret@gmail.com"),
        ..user2
    };
    user3.change_username("new_username".to_string());
    println!("User3 username: {}", user3.username);

    let string: String = "this is a String on the heap".to_string();
    println!("string to string: {}", string.to_string());
    format!("[formated: {}]", string); // format returns a String object
}
