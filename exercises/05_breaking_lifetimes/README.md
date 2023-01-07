# Breaking Lifetimes

Sometimes the best way to understand something is to break it. To that end,
in this chapter there are three exercises in which your goal is to *cause*
a compiler error.

## Exercise 1: Writing A Function With Bad Lifetimes

``` rust,ignore
fn longer(str1: &'a str, str2: &'b str) -> &'a str {}
```

## Exercise 2: Ownership error

This one requires a bit of explaining: Your function will be receiving
a reference which contains another reference.

``` rust,ignore
```

