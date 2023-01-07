# Lifetimes on Types

So far, we've only discussed lifetimes as applied to functions.
Functions are not the only place where you will need explicit
lifetimes. Types (structs and enums) can have lifetimes too.

This is because it

``` rust
struct Match<'a> {
    needle: &'a str,
    haystack: &'a str
}
```

``` rust
enum 
```
