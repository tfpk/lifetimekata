fn main() {
    let mut my_reference: Option<&i32> = None;

    // Starting a scope.
    {
        let my_variable: i32 = 7;
        my_reference = Some(&my_variable);
    }

    if let Some(reference) = my_reference {
        println!("{}", reference);
    }

}
