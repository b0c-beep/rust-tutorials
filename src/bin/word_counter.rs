use std::fs;
use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let filename = "data.txt";
    let contents = fs::read_to_string(filename)?;

    println!("File contents:\n{}", contents);

    let mut word_count = HashMap::new();

    for word in contents.split_whitespace() {
        let count = word_count.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    println!("\nWord Count:");
    for (word, count) in &word_count {
        println!("{}: {}", word, count);
    }

    Ok(())
}