pub fn select_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    if n < 2 {
        return nums;
    }

    for i in 0..n - 1 {
        let mut k = i;
        for j in i + 1..n {
            if nums[j] < nums[k] {
                k = j;
            }
        }

        swap(&mut nums, i, k);
    }
    nums
}

fn swap(nums: &mut [i32], i: usize, j: usize) {
    let n = nums.len();
    if i >= n || j >= n {
        panic!("our of index");
    }
    if i != j {
        let (max, min) = (i.max(j), i.min(j));
        let (left, right) = nums.split_at_mut(max);
        std::mem::swap(&mut left[min], &mut right[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let nums = vec![6, 4, 2, 9, 4, 8, 1, 3, 7, 2, 3, 5];

        let nums = select_sort(nums);

        assert_eq!(nums, vec![1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9]);

        let nums = vec![];
        let nums = select_sort(nums);
        assert_eq!(nums, vec![]);
    }
}
