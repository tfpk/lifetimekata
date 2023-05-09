# Lifetimes on Types

So far, we've only discussed lifetimes as applied to functions.
Functions are not the only place where you will need explicit
lifetimes. Types (structs and enums) can have lifetimes too.

This is because if a struct contains a reference, the user needs
to clarify how long it lasts for.

Imagine we wanted to split a `&str` in two, and create a
struct with a `start` and `end` field?

Well, we could write a function like this:

``` rust,ignore
struct SplitStr {
    start: &str,
    end: &str
}

fn split<'text, 'delim>(text: &'text str, delimiter: &'delim str) -> Option<SplitStr> {
    let (start, end) = text.split_once(delimiter)?;
    
    Some(SplitStr {
        start,
        end
    })
}

# fn main() {}
```

And we're done! Right?

Well, how long do those string references live?

What if we called the function like this:


``` rust,ignore
# struct SplitStr {
#     start: &str,
#     end: &str
# }
# 
# fn split<'text, 'delim>(text: &'text str, delimiter: &'delim str) -> Option<SplitStr> {
#     let (start, end) = text.split_once(delimiter)?;
#     
#     Some(SplitStr {
#         start,
#         end
#     })
# }

fn main() {
    let mut parts_of_string: Option<SplitStr> = None;
    {
        let my_string = String::from("First line;Second line");
        parts_of_string = split(&my_string, ";");
    }
    
    println!("{parts_of_string:?}");
}
```

Well, the references inside the `SplitStr` struct are now dangling,
since they both pointed to `my_string`; but that only existed inside the curly brackets.

So, Rust forces us to specify the lifetime of all references inside a struct.
Here's how we'd fix our code:

``` rust
struct SplitStr<'str_lifetime> {
    start: &'str_lifetime str,
    end: &'str_lifetime str
}

fn split<'text, 'delim>(text: &'text str, delimiter: &'delim str) -> Option<SplitStr<'text>> {
    let (start, end) = text.split_once(delimiter)?;
    
    Some(SplitStr {
        start,
        end
    })
}

# fn main() {}
```

Now, when we return an `Option<SplitStr<'text>>` the compiler knows that references inside the struct
must all last for the same lifetime as `'text`. If we try to return a `SplitStr` where the references
can't last for `'text`, that will be a compiler error.

## A Note on Enums

References work exactly the same way in enums as they do in structs.
We don't go into detail on them here because they are interchangeable.

``` rust
enum StringOption<'a> {
    Some(&'a str),
    None
}
# fn main() {}
```


## Two Lifetimes

Occasionally, structs will have more than one lifetime on them.
This happens when the data inside them comes from two different places,
with two lifetimes.

Take the example of a program to find unique words among two sentences.

You might have the first sentence be `"I love to swim and surf."`, and the
second be `"I love to ski and snowboard."`. The words unique to the first
sentence are `"swim"` and `"surf"`. The words unique to the second sentence are
`"ski"` and `"snowboard"`.

If you said that the two sentences had to share a lifetime, you would
be forcing the user to ensure that the two sentences came from the same
place, and therefore had the same lifetime. But what if one came from a file that
was open for the whole running of the program, but the second was scanned in
inside a loop?

In that case, the compiler would insist that the scanned in value was saved for
the whole of the program, which would not be ergonomic.

## Exercise: Two Lifetimes on a Struct

In this exercise, we will be modifying a small program which finds the unique
words between two strings. At the moment, it does not have any lifetime
annotations, and therefore does not compile.

Our goal is to return a struct that contains all the unique words from the
first string, and all the unique words from the second string. They should
have separate lifetimes.
