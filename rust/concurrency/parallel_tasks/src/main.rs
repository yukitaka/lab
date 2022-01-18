use rayon::prelude::*;

fn main() {
    mutate_the_elements_of_an_array_in_parallel();
}

fn mutate_the_elements_of_an_array_in_parallel() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}
