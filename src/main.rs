use std::io::stdin;

const OPTIONS: [&str; 3] = ["1. Add User", "2. View All Users", "3. Exit"];

struct User {
    username: String,
    email: String,
}

fn main() {
    println!("User Manager.");

    let mut running: bool = true;
    let mut users: Vec<User> = Vec::new();

    while running {
        println!("What would you like to do?");
        for option in OPTIONS {
            println!("{option}");
        }

        let input: String = read_user_input();

        match input.as_str() {
            "1" => users.push(add_user()),
            "2" => display_all_users(&users),
            "3" => running = false,
            _ => println!("Not a valid option"),
        }
    }
}

fn add_user() -> User {
    println!("Enter a username.");
    let username: String = read_user_input();
    println!("Enter an email.");
    let email: String = read_user_input();
    User {
        username,
        email,
    }
}

fn display_all_users(users: &Vec<User>) {
    for user in users.iter() {
        println!("Username: {}, Email: {}", &user.username, &user.email);
    }
}

fn read_user_input() -> String {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    String::from(input.trim())
}
