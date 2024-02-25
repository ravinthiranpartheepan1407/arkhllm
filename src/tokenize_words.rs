pub fn tokenize_words(x: &[&str]) -> Vec<String> {
    let mut token_words: Vec<String> = vec![];
    for (_, &elements) in x.iter().enumerate() {
        let split_words: Vec<&str> = elements.split_whitespace().collect();
        for word in split_words {
            token_words.push(word.to_string());
        }
    }
    token_words
}
