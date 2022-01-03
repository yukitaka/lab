use cslib::comb_sort;

fn main() {
    let mut numbers = vec![3, 4, 1];
    comb_sort(&mut numbers);
    println!("{:?}", numbers);
}
