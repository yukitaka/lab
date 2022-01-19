use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rayon::prelude::*;

mod thumb_in_parallel;

fn main() {
    mutate_the_elements_of_an_array_in_parallel();
    test_in_parallel_if_any_or_all_elements_of_a_collection_match_a_given_predicate();
    search_items_using_given_predicate_in_parallel();
    sort_a_vector_in_parallel();
    mapreduce_in_parallel();
    match thumb_in_parallel::generate_jpg_thumbnails_in_parallel() {
        Err(e) => {
            for e in e.iter() {
                println!("{}", e);
            }
        }
        _ => (),
    }
}

fn mutate_the_elements_of_an_array_in_parallel() {
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}

fn test_in_parallel_if_any_or_all_elements_of_a_collection_match_a_given_predicate() {
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| *n > 8));
    assert!(vec.par_iter().all(|n| *n <= 8));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| *n > 8));
    assert!(!vec.par_iter().all(|n| *n <= 8));

    println!("{:?}", vec);
}

fn search_items_using_given_predicate_in_parallel() {
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert!(f3 > Some(&8));
}

fn sort_a_vector_in_parallel() {
    let mut vec = vec![String::new(); 100_000];

    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = (0..5)
            .map(|_| char::from(rng.sample(&Alphanumeric)))
            .collect()
    });
    vec.par_sort_unstable();
}

struct Person {
    age: u32,
}

fn mapreduce_in_parallel() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 },
    ];

    let num_over_30 = v.par_iter().filter(|&x| x.age > 30).count() as f32;
    let sum_over_30 = v
        .par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(|| 0, |x, y| x + y);
    let alt_sum_30: u32 = v.par_iter().map(|x| x.age).filter(|&x| x > 30).sum();

    let avg_over_30 = sum_over_30 as f32 / num_over_30;
    let alt_avg_over_30 = alt_sum_30 as f32 / num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < f32::EPSILON);
    println!("The average age of people older than 30 is {}", avg_over_30);
}
