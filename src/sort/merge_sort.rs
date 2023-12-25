pub fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return nums;
    }
    let n = nums.len();
    _merge_sort(&mut nums, 0, n - 1);
    nums
}

fn _merge_sort(nums: &mut [i32], left: usize, right: usize) {
    if left >= right {
        return;
    }
    let mid = left + (right - left) / 2;
    _merge_sort(nums, left, mid);
    _merge_sort(nums, mid + 1, right);

    merge(nums, left, mid, right);
}

fn merge(nums: &mut [i32], left: usize, mid: usize, right: usize) {
    let temp_size = right - left + 1;
    let mut temp = vec![0; temp_size];

    let (mut i, mut j, mut k) = (left, mid + 1, 0);

    while i <= mid && j <= right {
        if nums[i] <= nums[j] {
            temp[k] = nums[i];
            i += 1;
        } else {
            temp[k] = nums[j];
            j += 1;
        }
        k += 1;
    }

    while i <= mid {
        temp[k] = nums[i];
        i += 1;
        k += 1;
    }
    while j <= right {
        temp[k] = nums[j];
        j += 1;
        k += 1;
    }

    for k in 0..temp_size {
        nums[left + k] = temp[k];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let nums = vec![6, 4, 2, 9, 4, 8, 1, 3, 7, 2, 3, 5];

        let nums = merge_sort(nums);

        assert_eq!(nums, vec![1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9]);

        let nums = vec![];
        let nums = merge_sort(nums);
        assert_eq!(nums, vec![]);
    }
}
