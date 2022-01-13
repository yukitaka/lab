fn main() {
    sort_a_vector_of_integers();
}

fn sort_a_vector_of_integers() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}
