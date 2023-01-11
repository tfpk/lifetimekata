# Finale

Congratulations on getting to the end of LifetimeKata. There's more footnotes and extra reading
in the next two chapters, but this chapter is a "finale" of sorts.

In this exercise, we will be building a very simple clone of a glob system.
This allows someone to ask whether a piece of text matches some description.

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
