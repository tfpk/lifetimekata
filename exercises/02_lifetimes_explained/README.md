## Okay, so what do we do?

The problem we've seen here is that when a function might output
a reference with a specific lifetime, it needs to know what it's
lifetime was (and therefore, from which inputs that lifetime came).

So, let's tell the compiler that certain references in the function signature
must both be valid over the same region of code. In other words,
they must share a lifetime.

Let's take a look at the syntax of this:

```rust
fn some_if_greater<'lifetime1, 'lifetime2>(number: &'lifetime1 i32, greater_than: &'lifetime2 i32) -> Option<&'lifetime1 i32> {
    if number > greater_than {
        Some(number)
    } else {
        None
    }
}
# fn main() {
#     let (n, gt) = (7, 4);
#     let test = some_if_greater(&n, &gt);
# }
```

There are two important components to notice here:

 - `fn  my_function<'lifetime1, 'lifetime2>(...)` this is written exactly the same way that generic parameters are.
   The only important thing to know is that all lifetime parameters must be declared before you declare types.
   So for a generic function you'd write `fn my_function<'lifetime1, T>(...)`
 - All references are written as `&'lifetime1 i32`. This is the same as a `&i32`, we've just given the compiler
   more information to understand the lifetimes.

What we're doing here is telling the compiler that it needs to find two regions of code:
`'lifetime1`  and `'lifetime2`. One region of code needs to contain everything that
lives as long as `'lifetime1`. Another region of code needs to contain everything
that lives as long as `'lifetime2`. If the compiler is unable to find a region
that matches that description, that is an error.

It's really important to note that we're not telling the compiler *what*
the region of code is. We're just telling it that it needs to find such
a region of code.


# Exercise: Annotate lifetimes

Just to get some initial practice, the exercise in this section is just to annotate the `split` function above
with lifetimes.
