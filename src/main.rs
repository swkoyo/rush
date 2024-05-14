use std::io::{self, Write};
use std::{env, fs};

const PREFIX: &str = "> ";

fn handle_pwd() {
    match env::current_dir() {
        Ok(n) => println!("{}", n.display()),
        Err(error) => println!("error: {error}"),
    };
}

fn handle_ls() {
    let entries = fs::read_dir("./")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join("  ");
    println!("{}", entries);
}

fn main() {
    loop {
        print!("{PREFIX}");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("ERR");

        let input = input.trim();

        if input == "pwd" {
            handle_pwd();
        } else if input == "ls" {
            handle_ls();
        }
    }
}
