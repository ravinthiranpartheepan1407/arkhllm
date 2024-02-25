pub fn to_lower(x: &[&str]) -> Vec<String> {
    let mut lower_data: Vec<String> = vec![];
    for (_, &element) in x.iter().enumerate() {
        let transformed_data: String = element.to_lowercase();
        lower_data.push(transformed_data.clone());
    }
    lower_data
}