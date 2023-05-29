# Lifetimes on Impls

When structs or enums have lifetimes on them, the way that `impl` blocks
work also changes slightly.

For example, say we want to create a struct which lets the user
iterate over a sentence. You might start off something like this:

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
        let start_of_word = &self.string[self.position..];
        let index_of_next_space = start_of_word.find(' ').unwrap_or(start_of_word.len());
        if start_of_word.len() != 0 {
            self.position += index_of_next_space + 1;
            Some(&start_of_word[..index_of_next_space]) 
        } else {
            None
        }
    }
}

fn main() {
    let text = String::from("Twas brillig, and the slithy toves // Did gyre and gimble in the wabe: // All mimsy were the borogoves, // And the mome raths outgrabe. ");
    let mut word_iterator = WordIterator::new(&text);
    
    assert_eq!(word_iterator.next_word(), Some("Twas"));
    assert_eq!(word_iterator.next_word(), Some("brillig,"));
    
}
```

When defining our `WordIterator` struct, we said it requires a lifetime to be specified.
But when we then wrote the impl block, we didn't specify one. Rust requires that we do this.

The way we do this is by telling Rust about a lifetime, and then putting that lifetime onto
our struct. Let's see how we do that:

``` rust,ignore
impl<'lifetime> for WordIterator<'lifetime> {
    // ...
}
```

It's useful to note that we've done this in two parts -- `impl<'lifetime>` defines a lifetime `'lifetime`.
It doesn't make any promises about what that lifetime is, it just says it exists.
`WordIterator<'lifetime>` then uses the lifetime we created, and says "the references in `WordIterator` must live for `lifetime`".

Now, anywhere in the impl block, we can choose to use that lifetime:

``` rust,ignore
# /// This struct keeps track of where we're up to in the string.
# struct WordIterator<'s> {
#     position: usize,
#     string: &'s str
# }

impl<'lifetime> WordIterator<'lifetime> {
    /// Creates a new WordIterator based on a string.
    fn new(string: &'lifetime str) -> WordIterator<'lifetime> {
        WordIterator {
            position: 0,
            string
        }
    }
    
    /// Gives the next word. `None` if there aren't any words left.
    fn next_word(&mut self) -> Option<&str> {
        let start_of_word = &self.string[self.position..];
        let index_of_next_space = start_of_word.find(' ').unwrap_or(start_of_word.len());
        if start_of_word.len() != 0 {
            self.position += index_of_next_space + 1;
            Some(&start_of_word[..index_of_next_space]) 
        } else {
            None
        }
    }
}

# fn main() {
#     let text = String::from("Twas brillig, and the slithy toves // Did gyre and gimble in the wabe: // All mimsy were the borogoves, // And the mome raths outgrabe. ");
#     let mut word_iterator = WordIterator::new(&text);
#     
#     assert_eq!(word_iterator.next_word(), Some("Twas"));
#     assert_eq!(word_iterator.next_word(), Some("brillig,"));
#     
# }

```

## Lifetime Elision, Redux

We previously discussed two rules for lifetime elision. They are:

1. Each place that an input lifetime is left out (a.k.a 'elided') is filled in with its own lifetime.
2. If there's exactly one lifetime on all the input references, that lifetime is assigned to *every* output lifetime.

Now that we've seen `impl` blocks that have lifetimes, let's discuss one more:

3. If there are multiple input lifetime positions, but one of them is `&self` or
   `&mut self`, the lifetime of the borrow of `self` is assigned to all elided output lifetimes.
   
This means that even if you take in many references in your arguments, Rust will assume that any references you return
come from `self`, not any of those other references.

# Exercise

In the following code, we chose to use the `'borrow` lifetime, not the `'lifetime` lifetime.

There are four ways we could implement this code. Describe the effect of each of these implementations.
Specifically:
 - Do they compile?
 - Are there any circumstances where their lifetimes are not general enough?
 - Which would be the "most" correct to write?

### Example 1
``` rust,ignore
    /// Gives the next word. `None` if there aren't any words left.
#    /// This compiles. It's the exact same as Example 4.
#    /// If you want to save the string you received, and call this again,
#    /// you cannot use this function, since the borrow needs to last
#    /// as long as the string.
    fn next_word<'borrow>(&'borrow mut self) -> Option<&'borrow str> {
        // ...
    }
```

### Example 2
``` rust,ignore
    /// Gives the next word. `None` if there aren't any words left.
#    /// This compiles. It's the exact same as Example 3.
    fn next_word<'borrow>(&'borrow mut self) -> Option<&'lifetime str> {
        // ...
    }
```

### Example 3
``` rust,ignore
    /// Gives the next word. `None` if there aren't any words left.
#    /// This compiles. It's probably the "most" correct, because it's the shortest
#    /// to write, but also ensures you can retain the returned strings, even if
#    /// you call this function multiple times.
    fn next_word(&mut self) -> Option<&'lifetime str> {
        // ...
    }
```

### Example 4
``` rust,ignore
    /// Gives the next word. `None` if there aren't any words left.
#    /// This compiles. If expanded, it would be the same as Example 1.
    fn next_word(&mut self) -> Option<&str> {
        // ...
    }
```
