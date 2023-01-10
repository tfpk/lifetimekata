use assert_could_be_elided::assert_could_be_elided;

/// This function returns the reference given to it.
///
/// Make sure it passes this test:
///
/// ```rust
/// use soln02::identity;
///
/// let x = 3;
/// assert_eq!(identity(&x), &x);
/// ````
#[assert_could_be_elided]
pub fn example_a<'a>(number: &'a i32) -> (&'a i32, &'a i32) {
    unimplemented!()
}

#[assert_could_be_elided]
pub fn example_b<'a, 'b, 'c, 'd>(first_arg: &'a i32, second_arg: &'b i32, third_arg: &'c Option<&'d i32> ) {
    unimplemented!()
}

#[assert_could_be_elided]
pub fn example_c<'a>(arg: &Option<&i32>) {
    unimplemented!()
}
