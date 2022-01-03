pub fn comb_sort(numbers: &mut Vec<i32>) {
    let mut interval = numbers.len() * 10 / 13;

    loop {
        let mut not_changed = true;
        let mut index = 0;
        let len = numbers.len();
        loop {
            if index + interval >= len {
                break;
            }
            if numbers[index] > numbers[index + interval] {
                numbers.swap(index, index + interval);
                not_changed = false;
            }
            index += 1;
        }
        if interval == 1 {
            if not_changed {
                return;
            }
        } else {
            interval = interval * 10 / 13;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comb_sort;

    #[test]
    fn it_sort() {
        let mut numbers = vec![3, 4, 9, 2, 1];
        comb_sort(&mut numbers);
        assert_eq!(vec![1, 2, 3, 4, 9], numbers);
    }
}
