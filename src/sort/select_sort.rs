pub fn select_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    for i in 0..n - 1 {
        let mut min = i;
        for j in i + 1..n {
            if nums[j as usize] < nums[min as usize] {
                min = j;
            }
        }
        (nums[min as usize], nums[i as usize]) = (nums[i as usize], nums[min as usize]);
    }
    nums
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
