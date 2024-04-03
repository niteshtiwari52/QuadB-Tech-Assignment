/*
   PRoblem :Check if a number is prime in Rust


*/

/*

     APPROACH :  Simple mathematics
        1--> Check if the number is less than 2. If it is, it's not prime. If the number is exactly 2, it's prime.

        2--> Iterate from 2 to the square root of the number (inclusive) and check if the number is divisible by any of these numbers.

        3--> If not divisible by any of the number then it would be prime
*/

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn main() {
    let num = 47;
    if is_prime(num) {
        println!("{} is prime", num);
    } else {
        println!("{} is not prime", num);
    }
}
