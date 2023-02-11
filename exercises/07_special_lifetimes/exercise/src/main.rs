use std::collections::HashSet;

struct UniqueWords<'a> {
    sentence: &'a str,
    unique_words: Vec<&'a str>,
}

impl<'a> UniqueWords<'a> {
    fn new(sentence: &'a str) -> UniqueWords<'a> {
        let unique_words = sentence
            .split(' ')
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        UniqueWords {
            sentence,
            unique_words,
        }
    }

    fn get_sorted_words<'b>(&'b self) -> Vec<&'a str> {
        let mut unique_words = self.unique_words.clone();
        unique_words.sort();
        unique_words
    }
}

fn main() {
    let words = UniqueWords::new("the hound and the fox liked the son of the fox");
    let sorted_words = words.get_sorted_words();
    println!("{}", words.sentence);
    println!("{sorted_words:?}");
}
