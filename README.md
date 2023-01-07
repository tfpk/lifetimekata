# LifetimeKata

Welcome to LifetimeKata, a set of exercises which you can use to improve your
understanding of lifetimes in Rust. While many tasks involve writing compiling
code, some will also involve creating specific errors.

You should complete the kata in order, as they increase in
difficulty, and depend on previous kata.

## Getting Started

Clone this repository:

``` sh
$ git clone https://www.github.com/tfpk/lifetimekata/
```

Build the main binary provided with this repo:

``` sh
$ cargo build --bin lifetimekata
```

You can find the first kata (`my_first_macro`) inside `exercises/01_my_first_macro`.
Read the `README.md` file and get started by editing the `main.rs` file.

To compare your expanded code to the "goal", use the `test` subcommand:

``` sh
$ cargo run -- test 01_my_first_macro
```

You can run your own code as follows:

``` sh
$ cargo run --bin 01_my_first_macro
```
