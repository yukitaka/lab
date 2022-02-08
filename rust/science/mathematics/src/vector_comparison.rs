use approx::assert_abs_diff_eq;
use ndarray::Array;

pub fn vector_comparison() {
    let a = Array::from_vec(vec![1., 2., 3., 4., 5.]);
    let b = Array::from_vec(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from_vec(vec![1., 2., 3., 4., 5.]);
    let mut d = Array::from_vec(vec![5., 4., 3., 2., 1.]);

    let z = a + b;
    let w = &c + &d;
    assert_abs_diff_eq!(z, Array::from_vec(vec![6., 6., 6., 6., 6.]));
    println!("c = {}", c);
    c[0] = 10.;
    d[1] = 10.;

    assert_abs_diff_eq!(w, Array::from_vec(vec![6., 6., 6., 6., 6.]));
}
