# Chapter 0: References and Lifetimes Review

*(This section is review, and may be familiar to some readers.
 If you already know what a reference is, you can skip it.)*

If you've been using Rust for a while, you've likely already encountered
references. For every type `T`, there are two types of references to it:

 - `&T`: A shared reference (often called a shared borrow) of `T`. You can have
   as many of these as you'd like, but they do not allow you to modify the 
   data they refer to.
 - `&mut T`: A mutable reference (often called an exclusive borrow) of `T`.
   You can only have one of these at a time, but they allow you to modify
   the data they refer to.

References make it easy to call a function on data
without needing to copy that data.

The powerful thing about Rust's references is that they are guaranteed to always
refer to something that still exists (i.e. has not been dropped/freed/gone out
of scope). A reference to something that no longer exists is called a "dangling
reference", and Rust guarantees you will never have one. Therefore, this example
will not compile:

```rust,ignore
fn main() {
    let x_ref = {
        let x = 3;
        &x
    };
    // x_ref would now refer to `x`, but `x` is out of scope, so x_ref is dangling.
   
    println!("{}", x_ref)
}
```

Most modern languages (Python, Java, etc.) avoid the problem of dangling references
by constantly checking at runtime whether you have any references to something, and
dropping only when you have no references left. This is
called "garbage collection", and the advantage of it is that you never need
to think about when objects dropped. The language just 
does it for you. The disadvantage is performance -- garbage collection requires
stopping your program occasionally for the language to scan every reference you have.

Some languages (notably C and Assembly) give you access to a
"pointer" type. Since pointers are raw addresses in memory, the compiler leaves
it to the programmer to ensure that they do not have dangling references. This
allows them to be used in memory constrained or performance critical
environments, but unfortunately means that a bug can access memory after it's
destroyed; resulting in a crash, or worse, a security issue.

Rust is powerful because it gives you the convenience of knowing at run-time
that you will never access freed memory; but the price you pay for this is
the compiler needing to be convinced at compile-time that you've
correctly used references.

## An Example of an Unconvinced Compiler

You've undoubtedly come across errors like the one below before:

```rust,ignore
fn main() {
    let mut my_reference: Option<&i32> = None;

    // Starting a scope.
    {
        // my_variable created                               // \ \
        let my_variable: i32 = 7;                            // | |
        my_reference = Some(&my_variable);                   // | |- my_variable exists here. ('variable)
        // At the end of the scope, `my_variable` is dropped // | |
        drop(my_variable);                                   // | |
        // my variable destroyed                             // | /
    }                                                        // | - my_reference needs to exist here. ('reference)
                                                             // |
    if let Some(reference) = my_reference {                  // |
        println!("{}", reference);                           // |
    }                                                        // /

}
```

```
error[E0597]: `my_variable` does not live long enough
  --> bad_lifetimes.rs:7:29
   |
7  |         my_reference = Some(&my_variable);
   |                             ^^^^^^^^^^^^ borrowed value does not live long enough
8  |     }
   |     - `my_variable` dropped here while still borrowed
9  |
10 |     if let Some(reference) = my_reference {
   |                              ------------ borrow later used here

error: aborting due to previous error; 1 warning emitted

```

Clearly in this example, since `my_variable` goes out of scope before `my_reference`,
it's possible that the `if let` could try and access `my_reference`, and find that
it's referencing a variable that no longer exists.

Rust says that this variable "does not live long enough". It notices that
"it's possible that `my_variable` is dropped before the reference stored in `my_reference`".

Formally, we can see this by noticing the regions of code where these two
things need to exist. The region of code where the reference needs to exist
is *larger* than the region of code where the variable exists. This indicates
that there must be part of the time the reference exists where the variable
has been dropped, and therefore a dangling reference might exist.

We call a region of code where a reference must be valid a "lifetime". We can
give lifetimes names using the syntax `'name`. So let us say that `'variable`
is the region of code where a reference to the variable is valid.
Also, let's say that `'reference` is the region of code where the reference
could be used. We can formally say that `'variable` must be larger than `'reference`.

This is obviously true, it is a shorthand for saying "the region of code where the reference is valid
must be larger than the region of code where the reference is actually usable". Consider
the opposite: if the reference was usable somewhere where the reference wasn't valid, you'd
have something that was *invalid*: unsound code, or in other words, a bug.

## So what's this book about then?

There are places where the Rust compiler is unable to figure out lifetimes, and
needs the programmer to explicitly specify this. This book is to help you
improve at writing explicit lifetimes (things like `&'a str`). And that starts
with the next chapter!

## Exercise: Complete the Rustlings Exercises on Lifetimes

If you aren't sure you understand the above, before reading on,
[complete the rustlings exercises about lifetimes](https://github.com/rust-lang/rustlings/tree/main/exercises/lifetimes).
