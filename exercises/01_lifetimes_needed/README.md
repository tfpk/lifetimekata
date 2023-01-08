# What are Manual Lifetimes?

In the last section, we discussed the concept of lifetimes as they apply to variables. In all those examples,
it was clear what region of code of a variable or reference existed in, based the on the curly brackets.
Manual lifetimes are used to help the compiler understand what's going on when it can't rely on scope
brackets (i.e. across function boundaries; and within structs and enums).

A good place to understand manual lifetime parameters is to start by
understanding why we actually need them. Lets work through some examples to see
why they exist:

The simplest possible example of a function that needs you to be explicit about
lifetimes is this one, which returns a reference to the larger of two integers.

``` rust,ignore
fn max_of_refs(a: &i32, b: &i32) -> &i32 {
    if *a > *b {
        a
    } else {
        b
    }
}
```

Imagine we call this function as follows:

``` rust,ignore
fn complex_function(a: &i32) -> &i32 {
    let b = 2;
    max_of_refs(a, &b)
}

fn main() {
    let a = 1;
    let my_num = complex_function(&a);
    println!("{my_num}");
}
```

If you work through this example, you will find that my_num would be a reference to a variable
from `complex_function` (which no longer exists). In other words, the lifetime of the return
value of `complex_function` will be longer than the lifetime of `b`.

Now, you might say, "but can't the compiler see at runtime that clearly this program won't work"?
Well, because we're using constants, yes the compiler probably could tell that this program won't work.

But what if we said `let a = rand::rand()` or `let b = read_number_from_stdin()`?
It's impossible for a compiler to tell whether this reference should be valid.

## Okay, why can't we just ban that case?

Your next thought might be "OK, surely all references of this type are unsound; lets just disallow them".
It would be worth being specific about what this ban is. The simplest ban would be "no references in function parameters",
but that might just be a little excessive (and entirely destructive to how useful rust is).

A more sensible ban which would cover this case would be: "Any function with
more than one reference input may not return a reference (or something
containing a reference)". This avoids the problem we've seen of being unclear on
where a reference is coming from. It would ban the above example.

But, how ergonomic would that be? What if you wanted a function like:

``` rust,ignore
fn only_if_greater(number: &i32, greater_than: &i32) -> Option<&i32> {
    if number > greater_than {
        Some(number)
    } else {
        None
    }
}
```

No matter the way in which you call this function, we *always* know that if our
return value is `Some`, it refers to `number`. It will never return a reference
to `greater_than`.

A more interesting example of this is a `split` function, which takes a string,
and returns a vector of slices of that string, split by some other string.

``` rust,ignore
fn split(text: &str, delimiter: &str) -> Vec<&str> {
    let mut last_split = 0;
    let mut matches: Vec<&str> = vec![];
    for i in 0..text.len() {
        if i < last_split {
            continue
        }
        if text[i..].starts_with(delimiter) {
            matches.push(&text[last_split..i]);
            last_split = i + delimiter.len(); 
        }
    }
    if last_split < text.len() {
        matches.push(&text[last_split..]);
    }
    
    matches
}
```

No matter how you call this function, it will always return a vector of slices from `text`,
never from `delimiter`.

## Ugh, but can't the compiler just figure this out?

At this point, you can probably notice that `matches.push` is only ever called with `text` slices.
So you might reasonably expect that the compiler could infer lifetimes automatically in this case.

It's possible that in simple cases it could. But your compiler might decide that it can't infer
lifetimes. Or it could succeed in inferring them... after 6 months.

So, the compiler needs more information. That information is provided by manual lifetimes.
Before we discuss them in detail, here is an exercise that will hopefully re-inforce the concepts,
before we deal with syntax.

## Exercise: Identify which programs work, and which break

Without using any lifetime syntax, answer the following questions for each of the code examples:

1. Which references could be returned by the function?
2. Under what circumstances (if any) might the function return a dangling reference?

NOTE: the code examples do not compile; you will need to read them and think about them.
You'll see how to make those exercises compile in the next chapter.

``` rust,ignore
```

