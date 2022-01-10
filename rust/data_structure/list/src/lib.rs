use crate::List::{Cons, Nil};
use std::fmt;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, car: u32) -> List {
        Cons(car, Box::new(self))
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.stringify())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::LinkedList;

    #[test]
    fn it_cons() {
        let mut a = List::new();
        a = a.prepend(1);
        a = a.prepend(2);

        assert_eq!("2, 1, Nil", a.stringify());
    }

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
