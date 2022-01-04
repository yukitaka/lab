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
    let llen = left.len();
    let rlen = right.len();
    let mut lidx = 0;
    let mut ridx = 0;

    loop {
        if lidx < llen {
            if ridx < rlen {
                if left[lidx] <= right[ridx] {
                    lidx += 1;
                } else {
                    left.splice(lidx..lidx, [right[ridx]]);
                    ridx += 1;
                    lidx += 1;
                }
            } else {
                lidx += 1;
            }
        } else if ridx < rlen {
            let mut add = true;
            for i in lidx..left.len() {
                if left[i] > right[ridx] {
                    left.splice(i..i, [right[ridx]]);
                    lidx += 1;
                    add = false;
                    break;
                }
            }
            if add {
                left.push(right[ridx]);
            }
            ridx += 1;
        } else if ridx == rlen {
            break;
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
