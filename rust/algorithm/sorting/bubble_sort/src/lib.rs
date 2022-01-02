pub fn bubble_sort_immutable(numbers: Vec<i32>) -> Vec<i32> {
    let mut sorted = numbers;
    let mut try_sort = true;
    while try_sort {
        try_sort = false;
        for i in 1..sorted.len() {
            if sorted[i - 1] > sorted[i] {
                sorted.swap(i - 1, i);
                try_sort = true;
            }
        }
    }
    sorted
}

pub fn bubble_sort_mutable(numbers: &mut Vec<i32>) {
    let mut try_sort = true;
    while try_sort {
        try_sort = false;
        for i in 1..numbers.len() {
            if numbers[i - 1] > numbers[i] {
                numbers.swap(i - 1, i);
                try_sort = true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bubble_sort_immutable;
    use crate::bubble_sort_mutable;

    #[test]
    fn it_immutable_sort() {
        assert_eq!(
            vec![1, 2, 3, 4, 9],
            bubble_sort_immutable(vec![3, 4, 9, 2, 1])
        );
    }

    #[test]
    fn it_mutable_sort() {
        let mut numbers = vec![3, 4, 9, 2, 1];
        bubble_sort_mutable(&mut numbers);
        assert_eq!(vec![1, 2, 3, 4, 9], numbers);
    }
}
