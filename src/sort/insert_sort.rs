pub fn insert_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    for i in 1..n {
        let base = nums[i];
        let mut j = (i - 1) as i32;
        while j >= 0 && nums[j as usize] > base {
            nums[(j + 1) as usize] = nums[j as usize];
            j -= 1;
        }
        nums[(j + 1) as usize] = base;
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let nums = vec![6, 4, 2, 9, 4, 8, 1, 3, 7, 2, 3, 5];

        let nums = insert_sort(nums);

        assert_eq!(nums, vec![1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9]);

        let nums = vec![];
        let nums = insert_sort(nums);
        assert_eq!(nums, vec![]);
    }
}
