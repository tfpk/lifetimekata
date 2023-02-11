# Finale

Congratulations on getting to the end of LifetimeKata. There's more footnotes and extra reading
in the next two chapters, but this chapter is a "finale" of sorts.

In this exercise, we will be building a very simple clone of a glob system.
This allows someone to ask whether a piece of text matches some description.

It's worth noting that implementing the whole thing may take up to an hour. If you just want to work on the lifetimes,
you can copy code from the `solution`, but this is a fun
and rewarding exercise to complete in its entirety.

For example, the glob `ab(cd|ef|gh)` matches any of the following strings: `abcd`, `abef`, `abgh`

You will create a `Matcher` struct, which has three fields:

 - A `&str`, representing the textual representation of our regex.
 - A `Vec<MatcherTokens>`, representing the different parts of the regex, in order.
 - An integer, keeping track of what the longest match for our regex was.

To create this, you'll take a string that looks like this: `hello.(town|world|universe).its.me`.
There are three components to this:

 - regular text, like 'hello', 'its' or 'me', which should only match that exact text
 - wildcards (the `.` character), which matches any single character.
 - optional text, like `(town|world|universe)`, which matches exactly one of a list of
   strings. So `(town|world|universe)` matches `town`, OR `world`, OR `universe`.
   
 These can be mixed and matched in any order (but you will never have one inside the other).
 With this string, you should create a vector of MatcherTokens which refer to the relevant
 parts of that string.
 
 You will then write a function which takes another string, and sees how much of the `Matcher`
 that particular string matches. You will return a vector of `(MatcherToken, &str)`, where the
 `MatcherToken` is the token that matched some text, and the `&str` is the text that was matched.
 
 
## An Example

Say you had the matcher `(Black|Bridge)(rock|stone|water).company`. This can be broken down into four parts:
 - `OneOfText(["Black", "Bridge"])`
 - `OneOfText(["rock", "stone", "water"])`
 - `Wildcard`
 - `RawText("company")`

Now, let's imagine we're given the following text: `BlackBridge`. `Black` matches the first token,
but `Bridge` does not match the second token.
So, we would return: `vec![(OneOfText(["Black", "Bridge"]), "Black")"]`. The most tokens we've matched is 1.

For a different example, take `Bridgestone_Tyres`.
`Bridge` matches the first matcher, `stone` matches
the second matcher, `_` matches the third matcher,
but `Tyres` doesn't match `company`. So the most tokens
we've matched is 3. We'd return a vec containing:
 
 - (`OneOfText(["Black", "Bridge"])`, `Bridge`)
 - (`OneOfText(["rock", "stone", "water"])`, `"stone"`)
 - (`Wildcard`, `"_"`)

### A Note On Unicode

Rust is able to deal with unicode characters (like emoji or Japanese Kanji) in its strings.
Of course, this increases the amount of complexity that is required for simple operations like
splitting a string into pieces, because it's possible to accidentally split a character in half.

The tests in the example *do not* use unicode, however if you want a "true" Rust experience,
change the tests to include a unicode character (an example is in the comments).
