# Special Lifetimes

There are two special lifetimes in Rust. It's worth discussing them both:

 - `'static`
 - `'_` (the implicit lifetime)

# The `static` lifetime

Some things in your program are guaranteed to last forever. The most common reason for this
is when they're information bundled inside your binary. For example, when you write
a program like this:

``` rust
fn main() {
    let my_text = "Hello World";
}
```

The text `"Hello World"` is actually somewhere inside the compiled binary. This means that
the reference to it is always valid, since the text is always there as long as the program
is running.

Therefore, if we were to talk about the type of the text, we'd say it's a `&'static str`.

Similarly, any references to a constant can also be `&'static`. For example:

``` rust

const SOME_COORDINATE: (i32, i32) = (7, 4);

fn main() {
    let static_reference: &'static (i32, i32) = &SOME_COORDINATE
}
```

# The `'_` lifetime (Anonymous Lifetimes, Placeholder Lifetimes)

The implicit lifetime tells Rust to figure out the lifetime
itself. There are three places where this lifetime is useful:

 - To simplify `impl` blocks
 - When consuming/returning a type that needs a lifetime
 - To write trait objects that contain references.

## Simplifying Impl Blocks

Say you're implementing a counter struct, that looks like this:

``` rust
struct Counter<'a> {
    counter: &'a mut i32
}

impl<'a> Counter<'a> {
    fn increment(&mut self) {
        self.counter += 1;
    }
}

fn main() {
    let mut num = 0;
    
    let mut counter = Counter { counter: &mut num };
    counter.increment();
    
    println!("{num}"); // prints 1
    
}
```

That's fine, but you'll notice that the `impl` block doesn't actually use the `'a` lifetime anywhere.
Therefore, we can simplify things by writing the following instead:

``` rust
impl Counter<'_> {
    fn increment(&mut self) {
        self.counter += 1;
    }
}
```

The two `impl` blocks above mean the same thing, but just take slightly fewer arguments.

## Returning Structs and Enums

This is recommended for the situation where you are returning a
struct/enum that contains a reference. You could write something like this:

``` rust

struct StrWrap<'a>(&'a str);

fn make_wrapper(string: &str) -> StrWrap {
    StrWrap(string)
}

# fn main() {}
```

But that syntax is no longer recommended, as you will see when you add the
`#![deny(rust_2018_idioms)]` annotation, where you get the error:

```text
error: hidden lifetime parameters in types are deprecated
 --> src/main.rs:8:34
  |
_ | fn make_wrapper(string: &str) -> StrWrap {
  |                                  ^^^^^^^ expected lifetime parameter
  |
note: the lint level is defined here
 --> src/main.rs:1:9
  |
_ | #![deny(rust_2018_idioms)]
  |         ^^^^^^^^^^^^^^^^
  = note: `#[deny(elided_lifetimes_in_paths)]` implied by `#[deny(rust_2018_idioms)]`
help: indicate the anonymous lifetime
  |
_ | fn make_wrapper(string: &str) -> StrWrap<'_> {
  |                                         ++++
```

By following the hint, it becomes clearer that `StrWrap` *does* contain a reference,
but that the compiler should just figure it out.

# Lifetimes on Trait Objects

See `12_lifetimes_on_trait_objects` for the gory details.

# Notes on Bad Information

The Rust Edition Guide previously contained a section
about anonymous lifetimes. The most popular google result
is now [this article](https://yegeun542.github.io/rust-edition-guide-ko/rust-2018/ownership-and-lifetimes/the-anonymous-lifetime.html) but I recommend disregarding it.

https://www.reddit.com/r/rust/comments/w06q7e/why_is_sometimes_used/

# Exercise

Use special lifetimes to remove all the lifetimes in this example.
