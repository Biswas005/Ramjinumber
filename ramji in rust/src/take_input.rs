//module to take user defined input

use std::io;

pub fn take_input() -> i32 
{
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input: i32 = input.trim().parse().expect("Please type a number!");
    return input;
}
