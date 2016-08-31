use std::env;
use std::path::Path;

fn increase(number: i32) {
    println!("{}", number + 1);
}

fn decrease(number: i32) {
    println!("{}", number - 1);
}

fn help(command: &str) {
    println!("Usage:
{} <string>
    Check whether given string is the answer.
{} {{increase|decrease}} <integer>
    Increase or decrease given integer by one.",
    command,
    command);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &str = Path::new(&args[0])
                              .file_name()
                              .and_then(|s| s.to_str())
                              .unwrap();

    match args.len() {
        1 => {
            println!("My name is '{}'. Try passing some arguments!", command);
        },
        2 => {
            match args[1].parse() {
                Ok(42) => println!("This is the answer!"),
                _      => println!("This is not the answer..."),
            }
        },
        3 => {
            let cmd = &args[1];
            let num = &args[2];
            let number: i32 = match num.parse() {
                Ok(n)  => n,
                Err(_) => {
                    println!("Error: second argument not an integer");
                    help(command);
                    return;
                },
            };

            // match &cmd[..] {
            match cmd.as_str() {
                "increase" => increase(number),
                "decrease" => decrease(number),
                _          => {
                    println!("Error: invalid command");
                    help(command);
                }
            }
        },
        _ => {
            help(command);
        },
    }
}
