/*
   PRoblem : Reverse a string in Rust


*/

/*
    Approach :
        1--> Create an empty string to store the reversed
        2--> iterate over the characters of the input string in reverse order
        3--> each character append to reversed string
        4--> return reversed string.
*/

fn reverse_string(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}

fn main() {
    let s = "Hello, world!";
    let reversed_s = reverse_string(s);
    println!("Original string: {}", s);
    println!("Reversed string: {}", reversed_s);
}
