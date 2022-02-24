pub fn return_type() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    let mut v3 = combine_vecs(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");
}

pub fn add_closure() {
    let plus_one = make_addr_function(1);
    assert_eq!(plus_one(2), 3);

    let plus_two = make_addr_function(2);
    assert_eq!(plus_two(2), 4);
}

pub fn impl_closure() {
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    assert_eq!(doubles.collect::<Vec<i32>>(), vec![4, 6]);
}

/*
use std::iter;
use std::vec::IntoIter;
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}
*/

fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn make_addr_function(y: i32) -> impl Fn(i32) -> i32 {
    move |x: i32| x + y
}

fn double_positives(numbers: &[i32]) -> impl Iterator<Item = i32> + '_ {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}
