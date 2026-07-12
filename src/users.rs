pub struct Users {
    pub username: String,
    pub password: String,
}

pub fn get_users() -> Vec<Users> {
    let mut users: Vec<Users> = Vec::new();

    users.push(Users {
        username: String::from("joao"),
        password: String::from("123456"),
    });

    users.push(Users {
        username: String::from("maria"),
        password: String::from("654321"),
    });

    users.push(Users {
        username: String::from("pedro"),
        password: String::from("senha123"),
    });

    users
}

fn _main() {
    println!("Hello Users");
}
