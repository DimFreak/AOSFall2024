fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    
    let mut max_word = String::new(); // To store the word with highest frequency
    let mut max_count = 0; // To store the highest frequency count

    for i in 0..words.len() {
        let mut count = 0;

        // Count occurrences of words[i]
        for word in &words {
            if *word == words[i] {
                count += 1;
            }
        }

        // Update max_word and max_count if current word has a higher frequency
        if count > max_count {
            max_word = words[i].to_string(); // Convert &str to String
            max_count = count;
        }
    }

    (max_word, max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}
