use std::collections::HashMap;

// Eq requires that you derive PartialEq on the type.
#[derive(PartialEq, Eq, Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str,
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str,
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_login<'a>(accounts: &Accounts<'a>,
                 username: &'a str,
                 password: &'a str) {
    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Attempting login...");

    let login = Account {
        username: username,
        password: password,
    };

    match accounts.get(&login) {
        Some(account_info) => {
            println!("Successful login!");
            println!("Name: {}", account_info.name);
            println!("EMail: {}", account_info.email);
        },
        _ => println!("Login failed!"),
    }
}

fn main() {
    let mut accounts: Accounts = HashMap::new();

    let account = Account {
        username: "j.everyman",
        password: "password123",
    };
    let account_info = AccountInfo {
        name: "John Everyman",
        email: "j.everyman@email.com",
    };
    accounts.insert(account, account_info);

    try_login(&accounts, "j.everyman", "psasword123");
    try_login(&accounts, "j.everyman", "password123");
}
