# Further Reading

The best source of information about lifetimes is the Rust reference.

 - [The Rust Reference (Lifetime Elision)](https://doc.rust-lang.org/reference/lifetime-elision.html)
 - [The Rust Reference (In General)](https://doc.rust-lang.org/reference/)

## Variance

once you're an expert, read up on variance.

## Brain Teaser 1: Why doesn't this program work:

This should be another way of implementing the code in exercise 5.
Unfortunately, it doesn't work. This took the author of this book
20 minutes to figure out (after having written 5 chapters), so he
challenges you to do better!

```
use std::collections::HashSet;

struct Difference<'first, 'second> {
    first_only: Vec<&'first str>,
    second_only: Vec<&'second str>
}

fn find_difference<'fst, 'snd>(sentence1: &'fst str, sentence2: &'snd str) -> Difference<'fst, 'snd> {
    let sentence_1_words: HashSet<&str> = sentence1.split(" ").collect();
    let sentence_2_words: HashSet<&str> = sentence2.split(" ").collect();

    Difference {
        first_only: (&sentence_1_words - &sentence_2_words).into_iter().collect(),
        second_only: (&sentence_2_words - &sentence_1_words).into_iter().collect(),
    }

}

fn main() {
    let first_sentence = String::from("I love the surf and the sand.");
    let second_sentence = String::from("I hate the surf and the sand.");

    let first_only = {
        let third_sentence = String::from("I hate the snow and the sand.");
        let diff = find_difference(&first_sentence, &third_sentence);
        diff.first_only
    };

    assert_eq!(first_only, vec!["hate", "surf"]);

    let second_only = {
        let third_sentence = String::from("I hate the snow and the sand.");
        let diff = find_difference(&third_sentence, &second_sentence);
        diff.second_only
    };

    assert_eq!(second_only, vec!["snow"]);
}
```

See https://github.com/rust-lang/rust/issues/73788 for more info.
