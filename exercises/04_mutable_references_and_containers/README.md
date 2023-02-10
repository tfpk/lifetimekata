# Mutable References and Containers

Mutable References work exactly the same way as regular references, with regards
to lifetime elision. The reason we have a chapter about them, however, is that if
you have a mutable reference, you might need to tell the compiler about lifetimes
even without a return value.

For example, let's take a look at this example:

``` rust,ignore
fn insert_value(my_vec: &mut Vec<&i32>, value: &i32) {
    my_vec.push(value);
}
```

We're not returning anything; so lifetimes don't matter, right?

Unfortunately, lifetimes are still important. The reference `value` actually needs to
live for the same time as the contents of the vector. If they didn't,
the vector might contain an invalid reference. For example, what would happen
in this scenario?

``` rust,ignore
fn insert_value(my_vec: &mut Vec<&i32>, value: &i32) {
    my_vec.push(value);
}

fn main() {
    let x = 1;
    let my_vec = vec![&x];
    {
        let y = 2;
        insert_value(&mut my_vec, &y);
    }
    println!("{my_vec:?}");
}
```

The reference to `y` in the above example is dangling when we try to print the vector!

We can use lifetimes to ensure that the two references live for the same amount of time:

``` rust
fn insert_value<'vec_lifetime, 'contents_lifetime>(my_vec: &'vec_lifetime mut Vec<&'contents_lifetime i32>, value: &'contents_lifetime i32) {
    my_vec.push(value)
}
fn main(){
    let mut my_vec = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1);
    insert_value(&mut my_vec, &val2);
    
    println!("{my_vec:?}");
}
```

This signature indicates that there are two lifetimes:

 - `'vec_lifetime`: The vector we've passed the function will need to live
   for a certain period of time.
 - `'contents_lifetime`: The contents of the vector need to live for a certain
   period of time. Importantly, the new `value` we're inserting needs to live
   for just as long as the contents of the vector. If they didn't, you would
   end up with a vector that contains an invalid reference.

## Do We Even Need Two Lifetimes?

You might wonder what happens if we don't provide two lifetimes. Does just
one lifetime work?

``` rust,ignore
fn insert_value<'one_lifetime>(my_vec: &'one_lifetime mut Vec<&'one_lifetime i32>, value: &'one_lifetime i32) {
    my_vec.push(value)
}

fn main(){
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1);
    insert_value(&mut my_vec, &val2);
    
    println!("{my_vec:?}");
}
```

No, it doesn't. We get two errors. Let's look at the first one:

```
error[E0499]: cannot borrow `my_vec` as mutable more than once at a time
  --> /tmp/rust.rs:11:18
   |
10 |     insert_value(&mut my_vec, &val1);
   |                  ----------- first mutable borrow occurs here
11 |     insert_value(&mut my_vec, &val2);
   |                  ^^^^^^^^^^^
   |                  |
   |                  second mutable borrow occurs here
   |                  first borrow later used here

```

This seems strange -- why can't you borrow `my_vec`?

Well, let's walk through what the compiler sees:

`&val` needs to last for as long as `my_vec` exists:

``` rust,ignore
# fn insert_value<'one_lifetime>(my_vec: &'one_lifetime mut Vec<&'one_lifetime i32>, value: &'one_lifetime i32) {
#     my_vec.push(value)
# }
# 
# fn main(){
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1); // \
    insert_value(&mut my_vec, &val2); // | - &val1 needs to last this long.
                                      // |
    println!("{my_vec:?}");           // /
# }
```

Whereas `&mut my_vec` only needs to last for the duration of `insert_value`.

``` rust,ignore
# fn insert_value<'one_lifetime>(my_vec: &'one_lifetime mut Vec<&'one_lifetime i32>, value: &'one_lifetime i32) {
#     my_vec.push(value)
# }
# 
# fn main(){
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1); // <- &mut my_vec only needs to last this long.
    insert_value(&mut my_vec, &val2); 
    
    println!("{my_vec:?}");
# }
```

But, we've told the compiler that it needs the borrows of both `&val1` and
`&mut my_vec` to share the same lifetime. So the compiler extends the borrow
of `&mut my_vec` to ensure they do share a lifetime:
It sees that if it let `&mut my_vec` live as long as `&val1`, it would
have that single region of code:

``` rust,ignore
# fn insert_value<'one_lifetime>(my_vec: &'one_lifetime mut Vec<&'one_lifetime i32>, value: &'one_lifetime i32) {
#     my_vec.push(value)
# }
# 
# fn main(){
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1); // \
    insert_value(&mut my_vec, &val2); // | - 'one_lifetime must be this region of code.
                                      // |
    println!("{my_vec:?}");           // /
# }
```

And that's fine. But now the compiler gets to the next line, and it sees you're
trying to borrow `&mut my_vec` again.

The compiler already decided `&mut my_vec` has to exist until the end of the function.
So now, you're asking it to create *two* mutable references... and that's not allowed.

So the compiler throws an error -- you're not allowed to borrow `&mut my_vec` again.


## Why does having two lifetimes fix this error?

Have a think before reading this section -- why does having two lifetimes
solve this bug?

Before, the compiler had to decide that `&mut my_vec` and `&val1` shared a lifetime.
In other words, that they lived as long as each-other.

By using two lifetimes, we've told the compiler that `&mut my_vec` and `&val1`
don't necessarily have to live for the same amount of time. And so,
it finds the following lifetimes:

``` rust,ignore
fn insert_value<'vec_lifetime, 'contents_lifetime>(my_vec: &'vec_lifetime mut Vec<&'contents_lifetime i32>, value: &'contents_lifetime i32) {
    my_vec.push(value)
}

fn main(){
    let mut my_vec: Vec<&i32> = vec![];
    let val1 = 1;
    let val2 = 2;
    
    insert_value(&mut my_vec, &val1); // <- 'vec_lifetime \
    insert_value(&mut my_vec, &val2); //                  | 'contents_lifetime
                                      //                  |
    println!("{my_vec:?}");           //                  /
}
```

## Exercise Part 1: The Other Error

First, let's look at the other error we got in the last section:

```
error[E0502]: cannot borrow `my_vec` as immutable because it is also borrowed as mutable
  --> /tmp/rust.rs:13:16
   |
10 |     insert_value(&mut my_vec, &val1);
   |                  ----------- mutable borrow occurs here
...
13 |     println!("{my_vec:?}");
   |                ^^^^^^
   |                |
   |                immutable borrow occurs here
   |                mutable borrow later used here
   |
```

Can you explain why this error occurs? Write it out in 50 words or less.

## Exercise Part 2: Writing Our Own

Add appropriate lifetimes to the function in the exercise.
