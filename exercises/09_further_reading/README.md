# Further Reading in the Rust Reference

The best sources of information about lifetimes are the Rust Reference and
Rustonomicon. If you require a really advanced knowledge of lifetimes to
complete a project, the reference will contain that knowledge. Most times,
however, if you feel you need to understand something complex about lifetimes,
you may find that there is a simpler alternative.

 - [The Rust Reference (Lifetime Elision)](https://doc.rust-lang.org/reference/lifetime-elision.html)
 - [The Rust Reference (In General)](https://doc.rust-lang.org/reference/)
 - [The Rustonomicon (Lifetimes)](https://doc.rust-lang.org/nomicon/lifetimes.html)

# Other Useful Lifetimes Content

- [Common Rust Lifetime Misconceptions](https://github.com/pretzelhammer/rust-blog/blob/master/posts/common-rust-lifetime-misconceptions.md)
- [Crust of Rust: Lifetime Annotations](https://www.youtube.com/watch?v=rAl-9HwD858)

## Variance and Subtyping

This guide does not cover the topic of "variance" at all, which is how lifetimes can be substituted for
one-another. Variance, while theoretically important, is not useful in a day-to-day understanding
of lifetimes, and so it was not included in the book.

You can read more about it in [the Rustonomicon (subtyping)](https://doc.rust-lang.org/nomicon/subtyping.html).

## Brain Teaser 1: Why doesn't this program work:

If you're interested in working through a really difficult exercise to test
your understanding of lifetimes and generics, the following exercise may be interesting.

This should be another way of implementing the code in exercise 5.
Unfortunately, it doesn't work. This took the author of this book
20 minutes to figure out (after having written five chapters), so he
challenges you to do better!

```rust,ignore
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

For more information about this issue, read [this Rust issue](https://github.com/rust-lang/rust/issues/73788).
