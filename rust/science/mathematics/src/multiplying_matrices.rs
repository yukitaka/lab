use ndarray::arr2;

pub fn multiplying_matrices() {
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let b = arr2(&[[6, 3], [5, 2], [4, 1]]);

    println!("Multiplying matrices");
    println!("{}", a.dot(&b));
}
