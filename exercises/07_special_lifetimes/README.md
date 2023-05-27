# Special Lifetimes and Bounds

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
    let static_reference: &'static (i32, i32) = &SOME_COORDINATE;
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
        *self.counter += 1;
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

``` rust,ignore
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

## Lifetimes on Trait Objects

See [Chapter 10: Footnote on Trait Lifetime Bounds](./chapter_10.md) for the gory details.

# Lifetime Bounds

Lifetime bounds are not widely used, so we don't devote a large section of these exercises to them.
You can probably skip this section unless you really want to know the details.

In short, they allow you to specify that one lifetime should outlive another. To specify one, use a where clause, such as 
`where 'a: 'b`.

To quote the Rust Reference: 

> Lifetime bounds can be applied to types or to other lifetimes.
> The bound `'a: 'b` is usually read as `'a` *outlives* `'b`.
> `'a: 'b` means that `'a` lasts at least as long as `'b`, so a reference `&'a ()` is valid whenever `&'b ()` is valid.

> ```rust,ignore
> fn f<'a, 'b>(x: &'a i32, mut y: &'b i32) where 'a: 'b {
>     y = x;                      // &'a i32 is a subtype of &'b i32 because 'a: 'b
>     let r: &'b &'a i32 = &&0;   // &'b &'a i32 is well formed because 'a: 'b
> }
> ```

> `T: 'a` means that all lifetime parameters of `T` outlive `'a`.
> For example, if `'a` is an unconstrained lifetime parameter, then `i32: 'static` and `&'static str: 'a` are satisfied, but `Vec<&'a ()>: 'static` is not.


# Exercise

You have been given code which contains many uses of the lifetimes `'a` and `'b'`.
All of these lifetimes can be replaced with either `'_` or `'static`.

Your task is to replace every occurance of the lifetimes `'a` and `'b` with either
`'_` or `'static`, to remove excessive lifetime declarations, and to ensure your 
code still compiles.

### Footnote on Out of Date Information
 
The Rust Edition Guide previously contained a section
about anonymous lifetimes. The most popular google result
is now [this article](https://yegeun542.github.io/rust-edition-guide-ko/rust-2018/ownership-and-lifetimes/the-anonymous-lifetime.html) but I recommend disregarding it, as it is out of date information.
