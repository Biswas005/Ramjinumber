//modular ramji number checker
//user defined modules
mod take_input;
mod slice_number;
mod factorial;
mod sum_of_factorials;
mod pause;


//main function to run the program
fn main() {
    println!("This program is used to take in a number and check if it is a ramji number or not");
    println!("Please enter a number");
    let number = take_input::take_input();
    let number_slice = slice_number::slice_number(number);
    let sum = sum_of_factorials::sum_of_factorials(number_slice);
    if sum == number 
    {
        println!("The number is a ramji number");
    }
    else 
    {
        println!("The number is not a ramji number");
    }
    pause::pause_cmd();
}
