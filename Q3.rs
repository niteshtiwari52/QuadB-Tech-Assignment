/*
    PROBLEM : Given a string of words, implement a function that returns the shortest word in the string.

*/

/*
    APPROACH :
            1--> Split the string into words and maintain a shortest word.

            2--> iterate over words of the string  and compare lenght of current word of the string and shortest word length.

            3--> Update the shortest word and return first available shortest word of the string.
*/
fn shortest_word(sentence: &str) -> Option<&str> {
    let mut shortest_word: Option<&str> = None;

    for word in sentence.split_whitespace() {
        if let Some(sw) = shortest_word {
            if word.len() < sw.len() {
                shortest_word = Some(word);
            }
        } else {
            shortest_word = Some(word);
        }
    }

    shortest_word
}

fn main() {
    let sentence = "Hello Every one I want to work with the team on react";
    match shortest_word(sentence) {
        Some(word) => println!("Shortest word: {}", word),
        None => println!("No words found"),
    }
}
