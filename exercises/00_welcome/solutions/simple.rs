#[derive(Debug)]
struct Number(i32);

#[derive(Debug)]
struct Coordinate {
    x: Number,
    y: Number,
    z: Number,
}


fn simple_example<'a, 'b>() {
    let my_coord = Coordinate {
        x: Number(1),
        y: Number(2),
        z: Number(3)
    };
    let x_ref: &'a Number = &my_coord.x;
    let y_ref: &'b Number = &my_coord.y;
    println!("{}", x_ref.0 + y_ref.0);
}


fn main() {
    simple_example();
}
