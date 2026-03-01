fn most_frequent_word(text: &str) -> (String, usize) {
    
    let mut words: Vec<(&str, usize)> = Vec::new();

    for word in text.split_whitespace() {
        let mut found = false;

        for entry in words.iter_mut() {
            if entry.0 == word {
                entry.1 += 1;
                found = true;
                break;
            }
        }

        if !found {
            words.push((word,1));
        }
    }

    let mut max_word = "";
    let mut max_count = 0;

    for (word, count) in words {
        if count > max_count {
            max_word = word;
            max_count = count;
        }
    }

    (max_word.to_string(), max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}