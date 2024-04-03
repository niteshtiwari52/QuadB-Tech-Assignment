/*
   PRoblem :Implement a function that checks whether a given string is a palindrome or not.


*/

/*
    Approach :

        1--> converting string into lowercase and removing alphanumeric from the string.
        2--> reversing the string
        3--> comparing given string and reversed string
        4--> return true if palindrome
*/

fn is_palindrome(s: &str) -> bool {
    let s = s.to_lowercase();
    let s = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    let reversed = s.chars().rev().collect::<String>();
    s == reversed
}

fn main() {
    // let input_string = "Hello!! My name is Nitesh Tiwari.";
    let input_string = "IJI!!";
    if is_palindrome(input_string) {
        println!("'{}' is a palindrome", input_string);
    } else {
        println!("'{}' is not a palindrome", input_string);
    }
}
