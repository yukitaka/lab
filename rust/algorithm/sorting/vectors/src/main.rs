fn main() {
    sort_a_vector_of_integers();
    sort_a_vector_of_floats();
    sort_a_vector_of_structs();
}

fn sort_a_vector_of_integers() {
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
}

fn sort_a_vector_of_floats() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn sort_a_vector_of_structs() {
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];

    people.sort();
    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]
    );

    people.sort_by(|a, b| b.age.cmp(&a.age));
    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]
    );
}
