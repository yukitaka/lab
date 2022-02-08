mod adding_matrices;
mod multiplying_matrices;
mod vector_comparison;
mod vector_norm;

fn main() {
    adding_matrices::adding_matrices();
    multiplying_matrices::multiplying_matrices();
    multiplying_matrices::multiply_a_scalar_with_a_vector_with_a_matrix();
    vector_comparison::vector_comparison();
    vector_norm::vector_norm();
}
