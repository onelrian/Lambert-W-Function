use std::io;

pub fn input(promt: &str) -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed To Read Input");

    input.trim().parse::<i32>().unwrap_or(0)
}
