pub fn comb_sort(numbers: &mut Vec<i32>) {
    let mut interval = numbers.len();

    loop {
        if interval > 1 {
            interval = interval * 10 / 13;
        }
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
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::comb_sort;
    use rand::{Rng, SeedableRng};

    #[test]
    fn it_sort() {
        let mut rng = rand::rngs::SmallRng::seed_from_u64(100);
        let mut numbers = vec![rng.gen()];
        for _ in 0..=100 {
            numbers.push(rng.gen());
        }
        let mut expect = numbers.clone();
        expect.sort();
        comb_sort(&mut numbers);
        assert_eq!(expect, numbers);
    }
}
