//module to slice a number in main program and return a vector

pub fn slice_number(number: i32) -> Vec<i32> 
{
    let mut number_slice: Vec<i32> = Vec::new();
    let mut number_copy = number;
    while number_copy > 0 
    {
        number_slice.push(number_copy % 10);
        number_copy = number_copy / 10;
    }
    return number_slice;

}