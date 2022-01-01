pub fn bubble_sort(numbers: Vec<i32>) -> Vec<i32> {
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

#[cfg(test)]
mod tests {
    use crate::bubble_sort;

    #[test]
    fn it_sort() {
        assert_eq!(vec![1, 2, 3, 4, 9], bubble_sort(vec![3, 4, 9, 2, 1]));
    }
}
