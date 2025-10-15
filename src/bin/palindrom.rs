use std::io;

fn main() {
    println!("Insert a word to check if it's a palindrome:");   

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if check_palindrome(&input.trim()) {
        println!("It's a palindrome!");
    } else {
        println!("It's not a palindrome.");
    }

}


fn check_palindrome(text: &str) -> bool {
    let cleaned_text: String = text
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();
    return cleaned_text == cleaned_text.chars().rev().collect::<String>();
}