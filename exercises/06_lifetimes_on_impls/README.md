# Lifetimes on Impls

When structs or enums have lifetimes on them, the way that `impl` blocks
work also changes slightly.

For example, say we want to create a struct which lets the user
skip through words. You might start off something like this:

``` rust,ignore
// First, the struct:

/// This struct keeps track of where we're up to in the string.
struct WordIterator<'s> {
    position: usize,
    string: &'s str
}

impl WordIterator {
    /// Creates a new WordIterator based on a string.
    fn new(string: &str) -> WordIterator {
        WordIterator {
            position: 0,
            string
        }
    }
    
    /// Gives the next word. `None` if there aren't any words left.
    fn next_word(&mut self) -> Option<&str> {
        let start_of_word = self.string[self.position..];
        let index_of_next_space = start_of_word.find(' ').unwrap_or(start_of_word.len());
        if start_of_word.len() != 0 {
            self.position += index_of_next_space + 1;
            Some(start_of_word[..index_of_next_space]) 
        } else {
            None
        }
    }
}

fn main() {
    let text = String::from("Twas brillig, and the slithy toves // Did gyre and gimble in the wabe: // All mimsy were the borogoves, // And the mome raths outgrabe. ");
    let mut word_iterator = WordIterator::new(&text);
    
    assert_eq!(word_iterator.next_word(), "Twas");
    assert_eq!(word_iterator.next_word(), "brillig,");
    
}
```

The tricky part here is what lifetime to assign to the `WordIterator` on the
line `impl WordIterator {`. Rust requires that we give a lifetime in this
circumstance. Furthermore, every `&self` or `&mut self` will need to have a
lifetime which relates to the lifetime of `WordIterator` {{footnote: this could
actually be done using Generic Associated Types and the Iterator trait, but
we're ignoring that for simplicity}}.
