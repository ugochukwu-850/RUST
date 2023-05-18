pub fn is_even(len: u64) -> bool {
    //own the len input
    if len % 2 == 0 {
        return true;
    }
    false
}
pub fn fizzbuz(mut n: i32) {
    while n >= 0 {
   

        if (n % 5 == 0) && (n % 3 == 0) {
            println!("Fizzbuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        }

        n -= 1;
    }
}
