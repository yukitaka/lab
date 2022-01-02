use bslib::bubble_sort_mutable;

fn main() {
    let mut numbers = vec![3, 4, 1];
    bubble_sort_mutable(&mut numbers);
    println!("{:?}", numbers);
}
