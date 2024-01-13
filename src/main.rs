use std::io::stdin;

const OPTIONS: [&str; 4] = ["1. Add User", "2. View All Users", "3. Remove User", "4. Exit"];

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
            "1" => add_user(&mut users),
            "2" => display_all_users(&users),
            "3" => remove_user(&mut users),
            "4" => running = false,
            _ => println!("Not a valid option"),
        }
    }
}

fn add_user(users: &mut Vec<User>) {
    println!("Enter a username.");
    let username: String = 'username_loop: loop {
        let input: String = read_user_input();
        for user in &mut users.iter() {
            if &user.username.to_ascii_lowercase() == &input.to_ascii_lowercase() {
                println!("Username already exists!");
                continue 'username_loop;
            }
        }
        break input
    };
    println!("Enter an email.");
    let email: String = read_user_input();
    users.push(User { username, email });
}

fn remove_user(users: &mut Vec<User>) {
    println!("Enter a username to remove.");
    let username: String = read_user_input();
    for (i, user) in &mut users.iter().enumerate() {
        if &user.username == &username {
            users.swap_remove(i);
            println!("Removed user: {}", &username);
            return;
        }
    }
    println!("User does not exist!");
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
