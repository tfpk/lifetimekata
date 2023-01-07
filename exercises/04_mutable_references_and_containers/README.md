# Mutable References and Containers

Mutable References work exactly the same way as regular references, with regards
to lifetime elision. The reason we have a chapter about them, however, is that if
you have a mutable reference, you might need to tell them compiler about lifetimes
even without a return value.

For example, let's take a look at this example:

``` rust,ignore
fn insert_value(my_vec: &mut Vec<&i32>, value: &i32) {
    my_vec.push(value);
}
```

We're not returning anything; so it's all good, right?

Unfortunately not. The reference `value` actually needs to
live as long as the contents of the vector. If they didn't,
the vector might contain an invalid reference.

So, in this case, we need to tell the compiler that `value` will
live at least as long as the contents of the vector. We can do this
with lifetimes.

``` rust,no_run
fn insert_value<'vec_lifetime, 'contents_lifetime>(my_vec: &'vec_lifetime mut Vec<&'contents_lifetime i32>, value: &'contents_lifetime i32) {
    my_vec.push(value)
}
# fn main(){}
```

This rather long function signature means that while the reference to the vec can last however long it needs to, the 
reference must live at least as long as the contents of the vec.

What's important here is that we didn't need to specify any output
to have the lifetimes be important.

## Exercise

Add appropriate lifetimes to the functions in the examples.
