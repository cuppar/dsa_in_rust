pub fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let right = nums.len() as i32 - 1;
    _quick_sort(&mut nums, 0, right);
    nums
}

pub fn _quick_sort(nums: &mut [i32], mut left: i32, mut right: i32) {
    while left < right {
        let pivot = partition(nums, left as usize, right as usize) as i32;

        if pivot - left < right - pivot {
            _quick_sort(nums, left, pivot - 1);
            left = pivot + 1;
        } else {
            _quick_sort(nums, pivot + 1, right);
            right = pivot - 1;
        }
    }
}

fn partition(nums: &mut [i32], left: usize, right: usize) -> usize {
    let median = median_three(nums, left, left + (right - left) / 2, right);
    (nums[left], nums[median]) = (nums[median], nums[left]);

    let pivot = nums[left];

    let (mut l, mut r) = (left, right);
    while l < r {
        while l < r && nums[r] >= pivot {
            r -= 1;
        }
        while l < r && nums[l] <= pivot {
            l += 1;
        }
        (nums[l], nums[r]) = (nums[r], nums[l]);
    }
    (nums[l], nums[left]) = (nums[left], nums[l]);
    l
}

fn median_three(nums: &mut [i32], left: usize, mid: usize, right: usize) -> usize {
    if (nums[left] < nums[mid]) ^ (nums[left] < nums[right]) {
        left
    } else if (nums[mid] < nums[left]) ^ (nums[mid] < nums[right]) {
        mid
    } else {
        right
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
