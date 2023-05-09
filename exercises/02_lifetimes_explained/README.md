# Lifetimes: The Solution to Dangling References

What we can see here is that dangling references are caused by functions losing
information about where references come from.

The compiler can only decide if the function is correct if it
knows how the lifetimes of its inputs and outputs interact.
So, we need to tell the compiler when lifetimes of inputs and
outputs will be the same.

What we can do is tell the computer "my function works for any lifetimes,
as long as the lifetimes of these inputs/outputs are the same".
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

Let's walk through what this does:

 - `fn my_function<'lifetime1, 'lifetime2>(...)`: what we're doing here
   is choosing some names for the lifetimes our program requires.
 - `number: &'lifetime1 i32`: this is us telling the compiler that
   this reference must live for some region of code called `'lifetime1`.
 - `greater_than: &'lifetime2 i32`: this is us telling the compiler that
   this reference must live for some region of code called `'lifetime2`.
   This means that the lifetimes of `greater_than` and `number` don't
   have to relate at all.
 - `-> Option<&'lifetime1 i32>`: this is where lifetimes are important.
   what we're saying is that `number` and our return value must be
   valid for exactly the same region of code. 

So, what we've done is told the compiler that our function can only be called
if `number` and the return are valid in the same region of code.

# Exercise: Annotate lifetimes

Just to get some initial practice, the exercise in this section is to annotate lifetimes on some of the examples of the last two chapters.

You will need to:
 - decide how many lifetime parameters are necessary
 - name each of those lifetime parameters, and put them inside `<` angle brackets `>` after the function's name. 
 - annotate every reference with the appropriate lifetime
 - check the code compiles
 - think about what region of code each lifetime could be
 
You will notice each function has the `#[lifetimes_required(!)]` annotation. You will need
to leave it there to complete this exercise. This instructs the compiler to throw an
error whenever you miss a lifetime; even if the compiler doesn't need the lifetime.

