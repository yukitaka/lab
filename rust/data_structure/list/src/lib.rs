#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    #[test]
    fn it_join() {
        let mut a = LinkedList::new();
        a.push_back(1);
        a.push_back(2);
        a.push_back(3);
        let mut b = LinkedList::new();
        b.push_back(4);
        b.push_back(5);
        a.append(&mut b);

        let expect = LinkedList::from([1, 2, 3, 4, 5]);
        assert_eq!(expect, a);
    }
}