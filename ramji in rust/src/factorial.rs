//module to have factorial function in rust


use std::collections::HashMap;

pub fn factorial(n: i32, memo: &mut HashMap<i32, i32>) -> i32
 {
    if n == 0 
    {
        return 1; // 0! is defined as 1
    }
    
    // Check if the result is already memoized
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    
    // Calculate the result using recursion
    let result = n * factorial(n - 1, memo);
    
    // Store the result in the memoization HashMap
    memo.insert(n, result);
    
    result
}
