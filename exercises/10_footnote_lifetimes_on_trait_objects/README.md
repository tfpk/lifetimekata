# Footnote: Lifetimes on Trait Objects

In chapter 7, we discussed placeholder lifetimes (`'_`). We said that
there were three ways you could use them:

 - To simplify `impl` blocks
 - When consuming/returning a type that needs a lifetime
 - To write trait objects that contain references.

In the first case, we saw that anonymous lifetimes just simplified
what we needed to write.

In the second case, we saw that Rust recommends that we use it, but
we don't *need* to -- lifetime elision will do what we want.

The one case where it looks like lifetime elision should do what we want,
but it actually doesn't unless we use the `'_` is the case of trait objects.
This chapter walks through how trait objects and lifetimes work together.

Let's setup a simple example:

```rust
trait Bool {
    fn truthiness(&self) -> bool;
}

struct True();
impl Bool for True {
    fn truthiness(&self) -> bool {
        true
    }
}

struct False();
impl Bool for False {
    fn truthiness(&self) -> bool {
        false
    }
}

fn get_bool(b: bool) -> Box<dyn Bool> {
    if b == true {
        Box::new(True())
    } else {
        Box::new(False())
    }
}

fn main() {
    let my_bool = true;
    let my_dyn = get_bool(my_bool);

    println!("{}", my_dyn.truthiness());
}
```

To be clear, what we are doing here is creating two structs which represent
`true` and `false`. They both implement the `Bool` trait, which has
the `truthiness` function which returns `true` or `false`.

The `get_bool` function returns a Boxed `Bool` trait object, based on whether
`get_bool` is passed `true` or  `false`.

It's important to realise that since trait objects might or might not contain a
reference (or any number of references), all trait objects have lifetimes.
This is true, even if no implementors of the trait contain references.{{footnote:https://doc.rust-lang.org/reference/types/trait-object.html#trait-object-lifetime-bounds}}

So, since we need to associate a lifetime with our trait object, we might
think we could rely on lifetime elision. But how would lifetime elision
work for our `get_bool` function? There are no input references, so what
output lifetime should we give the trait object? Lifetime elision can't help
us here.

So, in RFC 599 and in RFC 1156, the rules for trait object lifetimes were changed.
The rules are complex, and best outlined [in the reference](https://doc.rust-lang.org/reference/lifetime-elision.html#default-trait-object-lifetimes),
but in the case of `get_bool`, it means that the lifetime inferred for `dyn Bool` is
`'static`.

Let's change the example slightly now, such that the struct contains a reference
to a bool:

```rust,ignore
trait Bool {
    fn truthiness(&self) -> bool;
}

// CHANGE 1: added &'a bool here
struct True<'a>(&'a bool);
impl<'a> Bool for True<'a> {
    fn truthiness(&self) -> bool {
        true
    }
}

// CHANGE 2: added &'a bool here
struct False<'a>(&'a bool);
impl<'a> Bool for False<'a> {
    fn truthiness(&self) -> bool {
        false
    }
}

fn get_bool(b: &bool) -> Box<dyn Bool> {
    if *b == true {
        Box::new(True(b))
    } else {
        Box::new(False(b))
    }
}

// CHANGE 3: Update the 
fn main() {
    let my_dyn = {
        let my_bool = true;
        get_bool(&my_bool)
        // my_bool is dropped here, so the trait object we're returning
        // has a dangling reference.
    };
    println!("{}", my_dyn.truthiness());
}
```

Now, we get an error:

```
error: lifetime may not live long enough
  --> src/main.rs:22:5
   |
21 |   fn get_bool(b: &bool) -> Box<dyn Bool> {
   |                  - let's call the lifetime of this reference `'1`
22 | /     if *b == true {
23 | |         Box::new(True(b))
24 | |     } else {
25 | |         Box::new(False(b))
26 | |     }
   | |_____^ returning this value requires that `'1` must outlive `'static`
   |
help: to declare that the trait object captures data from argument `b`, you can add an explicit `'_` lifetime bound
   |
21 | fn get_bool(b: &bool) -> Box<dyn Bool + '_> {
   |                                       ++++

error: could not compile __ due to previous error

```

Even though lifetime elision means that `get_bool` should end up with a
signature like `fn get_bool<'elided>(b: &'elided bool) -> Box<dyn Bool +
'elided>`, it doesn't. The special rules for trait objects mean that the
lifetime is: `fn get_bool<'elided>(b: &'elided bool) -> Box<dyn Bool +
'static>`. That `'static` bound is incorrect.

Therefore, we need the `'_` bound (as this error message tells us) to inform Rust that it
should use the normal lifetime elision rules; rather than the special trait
object rules.
