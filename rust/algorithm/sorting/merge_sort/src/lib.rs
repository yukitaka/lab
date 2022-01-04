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
                }
            } else {
                lidx += 1;
            }
        } else if ridx < rlen {
            let last = left.len() - 1;
            if left[last] >= right[ridx] {
                left.splice(last..last, [right[ridx]]);
            } else {
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

    #[test]
    fn it_sort() {
        let mut numbers = vec![3, 2, 4, 7];
        merge_sort(&mut numbers);

        assert_eq!(vec![2, 3, 4, 7], numbers);
    }
}