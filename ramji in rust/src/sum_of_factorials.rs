//module to have sum of factorials function in rust using memoization
use std::collections::HashMap;
use crate::factorial::factorial;

pub fn sum_of_factorials(number_slice: Vec<i32>) -> i32 
{
    
    let mut sum: i32 = 0;
    for i in number_slice 
    {
        let mut memo: HashMap<i32, i32> = HashMap::new();
        let result = factorial(i, &mut memo);
        sum = sum + result;
    }
    return sum;
}
