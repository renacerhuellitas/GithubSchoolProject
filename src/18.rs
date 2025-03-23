use std::collections::HashSet;

fn main() {
    let input: Vec<char> = String::from("abcde").chars().collect();
    let target_set: HashSet<char> = ['a', 'b', 'c'].iter().cloned().collect();

    if input.iter().any(|&char| target_set.contains(char)) {
        println!("Input contains target set elements");
    } else {
        println!("Input does not contain target set elements");
    }
}
