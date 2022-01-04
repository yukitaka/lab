use merge_sort::merge_sort;

fn main() {
    let mut numbers = vec![3, 1, 2, 4, 7, 5, 2];
    merge_sort(&mut numbers);

    println!("res: {:?}", numbers);
}
