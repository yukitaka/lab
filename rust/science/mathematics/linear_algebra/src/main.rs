mod adding_matrices;
mod deserialize_matrix;
mod invert_matrix;
mod multiplying_matrices;
mod vector_comparison;
mod vector_norm;

fn main() {
    adding_matrices::adding_matrices();
    multiplying_matrices::multiplying_matrices();
    multiplying_matrices::multiply_a_scalar_with_a_vector_with_a_matrix();
    vector_comparison::vector_comparison();
    vector_norm::vector_norm();
    invert_matrix::invert_matrix();
    if let Err(e) = deserialize_matrix::deserialize_a_matrix() {
        println!("{}", e);
    }
}
