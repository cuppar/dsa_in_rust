pub fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;
    _quick_sort(&mut nums, 0, n - 1);
    nums
}

fn _quick_sort(nums: &mut Vec<i32>, left: i32, right: i32) {
    if left >= right {
        return;
    }

    let pivot = partition(nums, left as usize, right as usize) as i32;

    _quick_sort(nums, left, pivot - 1);
    _quick_sort(nums, pivot + 1, right);
}

fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
    let mid = median_three(nums, left, left + (right - left) / 2, right);
    swap(nums, mid, left);

    let pivot = nums[left];
    let (mut l, mut r) = (left, right);
    while l < r {
        while l < r && nums[r] >= pivot {
            r -= 1;
        }
        while l < r && nums[l] <= pivot {
            l += 1;
        }
        swap(nums, l, r);
    }
    swap(nums, left, l);

    l
}

fn median_three(nums: &mut Vec<i32>, left: usize, mid: usize, right: usize) -> usize {
    if (nums[left] < nums[mid]) ^ (nums[left] < nums[right]) {
        left
    } else if (nums[mid] < nums[left]) ^ (nums[mid] < nums[right]) {
        mid
    } else {
        right
    }
}

fn swap(nums: &mut Vec<i32>, i: usize, j: usize) {
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

        let nums = quick_sort(nums);

        assert_eq!(nums, vec![1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9]);

        let nums = vec![];
        let nums = quick_sort(nums);
        assert_eq!(nums, vec![]);
    }
}
