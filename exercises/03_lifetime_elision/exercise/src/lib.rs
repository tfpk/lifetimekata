use require_lifetimes::require_lifetimes;

#[require_lifetimes(!)]
pub fn example_a(_number: &i32) -> (&i32, &i32) {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_b(_first_arg: &i32, _second_arg: &i32, _third_arg: &Option<&i32>) {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_c<'a>(_first_arg: &'a i32, _second_arg: &'a i32) -> &i32 {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_d<'a>(_first_arg: &'a i32, _second_arg: &i32) -> &'a i32 {
    unimplemented!()
}
