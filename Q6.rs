/*
   PRoblem : mplement a function that finds the longest common prefix of a given set of strings.

*/

/*
    Approach :
        1-->  Start with an empty string as the initial common prefix.
        2--> Iterate Through Characters: Iterate through the characters of the first string in the set. For each character position:
            - Check if all strings in the set have the same character at that position.

            - If they do, append the character to the common prefix.-
            - If not, stop iterating.

        3--> Return the Prefix: After iterating through all characters, return the common prefix.
*/
fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new(); // If the input set is empty, return an empty string
    }

    let mut prefix = String::new();
    let first_str = &strs[0]; // Get the first string in the set

    for (i, &ch) in first_str.as_bytes().iter().enumerate() {
        for s in strs.iter().skip(1) {
            if i >= s.len() || s.as_bytes()[i] != ch {
                return prefix;
            }
        }
        prefix.push(ch as char);
    }

    prefix
}

fn main() {
    let strings = vec![
        String::from("frontend"),
        String::from("from"),
        String::from("frog"),
    ];

    println!("Longest common prefix: {}", longest_common_prefix(&strings));
}
