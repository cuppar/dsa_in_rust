pub fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    // [0, i] will be process
    for i in (1..n).rev() {
        let mut exchanged = false;
        for j in 0..i {
            if nums[j] > nums[j + 1] {
                (nums[j], nums[j + 1]) = (nums[j + 1], nums[j]);
                exchanged = true;
            }
        }
        if !exchanged {
            break;
        }
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let nums = vec![6, 4, 2, 9, 4, 8, 1, 3, 7, 2, 3, 5];

        let nums = bubble_sort(nums);

        assert_eq!(nums, vec![1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9]);

        let nums = vec![];
        let nums = bubble_sort(nums);
        assert_eq!(nums, vec![]);
    }
}
