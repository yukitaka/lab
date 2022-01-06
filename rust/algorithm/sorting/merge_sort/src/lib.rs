pub fn merge_sort(numbers: &mut Vec<i32>) {
    let mid = numbers.len() / 2;
    if mid == 0 {
        return;
    }

    let mut right = numbers.split_off(mid);
    merge_sort(numbers);
    merge_sort(&mut right);

    merge(numbers, &right);
}

fn merge(left: &mut Vec<i32>, right: &Vec<i32>) {
    let tmp = left.clone();
    let first = tmp.as_slice();
    let second = right.as_slice();
    let first_len = first.len();
    let second_len = second.len();

    let mut i = 0;
    let mut j = 0;
    left.clear();

    for _ in 0..(first_len + second_len) {
        for _ in i..first_len {
            if j == second_len {
                left.push(first[i]);
                i += 1;
                continue;
            }
            for _ in j..second_len {
                if first[i] > second[j] {
                    left.push(second[j]);
                    j += 1;
                } else {
                    left.push(first[i]);
                    i += 1;
                    break;
                }
            }
        }
    }
    if j < second_len {
        for k in j..second_len {
            left.push(second[k]);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::merge_sort;
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
        merge_sort(&mut numbers);

        assert_eq!(expect, numbers);
    }
}
