fn main() {
    calculating_the_side_length_of_a_triangle();
    verifying_tan_is_equal_to_sin_divided_by_cos();
    distance_between_two_points_on_the_earth();
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

fn distance_between_two_points_on_the_earth() {
    let earth_radius_kilometer = 6371.0_f64;
    let (paris_latitude_degrees, paris_longitude_degrees) = (48.85341_f64, -2.34800_f64);
    let (london_latitude_degrees, london_longitude_degrees) = (51.50853_f64, -0.12574_f64);

    let paris_latitude = paris_latitude_degrees.to_radians();
    let london_latitude = london_latitude_degrees.to_radians();

    let delta_latitude = (paris_latitude_degrees - london_latitude_degrees).to_radians();
    let delta_longitude = (paris_longitude_degrees - london_longitude_degrees).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2)
        + paris_latitude.cos() * london_latitude.cos() * (delta_longitude / 2.0).sin().powi(2);
    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;

    println!(
        "Distance between Paris and London on the surface of Earth is {:.1} kilometers",
        distance
    );
}
