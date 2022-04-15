use ref_with_flag::ref_with_flag::RefWithFlag;

fn main() {
    let vec = vec![10, 20, 30];
    let flagged = RefWithFlag::new(&vec, true);
    assert_eq!(flagged.get_ref()[1], 20);
    assert!(flagged.get_flag());
}
