fn main() {
    calculating_the_side_length_of_a_triangle();
}

fn calculating_the_side_length_of_a_triangle() {
    let angle: f64 = 2.0;
    let side_length = 80.0;
    let hypotenuse = side_length / angle.sin();

    println!("Hypotenuse: {}", hypotenuse);
}
