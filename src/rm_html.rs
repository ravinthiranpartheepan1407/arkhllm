// extern crate regex;
// use regex::Regex;
// const X: [&str; 2] = ["<h1>Class Name</h1><br /><div><p>Team A</p></div>","<h1>Class Name</h1><br /><div><p>Team B</p></div>"];
// pub fn remove_html(x: &[&str]){
//     let mut html_tags: Vec<&str> = vec![];
//     let find_tags = Regex::new(r"<[^]*>").unwrap();
//     for(idx, &elements) in x.iter().enumerate(){
//         let pop_tags: Vec<&str> = find_tags.find_iter(&elements).map(|m| m.as_str()).collect();
//         html_tags.extend(pop_tags);
//         println!("Vectors: {} : {}",idx,elements);
//     }
// }

// fn main(){
//     let rm_html = remove_html(&X);
//     println!("Strings: {:?}",rm_html);
// }