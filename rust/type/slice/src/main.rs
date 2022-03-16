fn main() {
    comparison();
}

fn comparison() {
    let vec = vec![1, 2, 3];
    let vecref = &vec;
    let ary = [1, 2, 3];
    let aryref = &ary;

    assert_eq!(vecref, &vec![1, 2, 3]);
    assert_eq!(vecref, &[1, 2, 3]);
    assert_eq!(vecref, &ary);
    assert_eq!(aryref, &ary);
    // assert_eq!(&ary, &vec); the trait PartialEq<Vec<i32>> is not implemented for [i32; 3]
}
