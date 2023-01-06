fn count_shared_letters(string1: &str, string2: &str) -> usize {
    use std::collections::HashSet;
    let first_set = string1.chars().into::<HashSet<_>>();
    let second_set = string2.chars().into::<HashSet<_>>();
    first_set.intersection(second_set).count()
}

fn main() {
    let first_line = String::from("Two roads diverged in a yellow wood.");
    let shared_letters = {
        let second_line = String::from("A sky full of stars.");
        count_shared_letters(&first_line, &second_line)
    };

    println!("The two strings share {shared_letters} letters");
}
