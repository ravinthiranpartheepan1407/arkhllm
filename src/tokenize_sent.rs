pub fn tokenize_sentence(x: &[&str]) -> Vec<String>{
    let mut split_sent: Vec<String> = vec![];
    for(_, &elements) in x.iter().enumerate(){
        let split_sentence: Vec<&str> = elements.split_whitespace().collect();
        let split: String = split_sentence.join(" ");
        split_sent.push(split);
    }
    return split_sent;
}