fn main() {
    calculating_the_side_length_of_a_triangle();
    verifying_tan_is_equal_to_sin_divided_by_cos();
}

fn calculating_the_side_length_of_a_triangle() {
    let angle: f64 = 2.0;
    let side_length = 80.0;
    let hypotenuse = side_length / angle.sin();

    println!("Hypotenuse: {}", hypotenuse);
}

fn verifying_tan_is_equal_to_sin_divided_by_cos() {
    let x: f64 = 6.0;
    let a = x.tan();
    let b = x.sin() / x.cos();

    assert_eq!(a, b);
}
