use require_lifetimes::require_lifetimes;

#[require_lifetimes(!)]
pub fn example_a<'a>(_number: &'a i32) -> (&'a i32, &'a i32) {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_b<'a, 'b, 'c, 'd>(
    _first_arg: &'a i32,
    _second_arg: &'b i32,
    _third_arg: &'c Option<&'d i32>,
) {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_c<'a>(_first_arg: &'a i32, _second_arg: &'a i32) -> &'a i32 {
    unimplemented!()
}

#[require_lifetimes(!)]
pub fn example_d<'a, 'b>(_first_arg: &'a i32, _second_arg: &'b i32) -> &'a i32 {
    unimplemented!()
}
