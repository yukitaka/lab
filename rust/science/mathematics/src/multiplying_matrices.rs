use ndarray::{arr1, arr2, Array1};

pub fn multiplying_matrices() {
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let b = arr2(&[[6, 3], [5, 2], [4, 1]]);

    println!("Multiplying matrices");
    println!("{}", a.dot(&b));
}

pub fn multiply_a_scalar_with_a_vector_with_a_matrix() {
    let scalar = 4;
    let vector = arr1(&[1, 2, 3]);
    let matrix = arr2(&[[4, 5, 6], [7, 8, 9]]);

    println!("Multiply a scalar with a vector with a matrix");
    let new_vector: Array1<_> = scalar * vector;
    println!("{}", new_vector);

    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);
}
