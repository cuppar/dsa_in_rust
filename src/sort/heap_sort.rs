pub fn heap_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32;

    for i in (0..=(n - 1) / 2).rev() {
        sift_down(&mut nums, n as usize, i as usize);
    }

    for i in (1..n).rev() {
        (nums[i as usize], nums[0]) = (nums[0], nums[i as usize]);

        sift_down(&mut nums, i as usize, 0);
    }

    nums
}

fn sift_down(nums: &mut [i32], n: usize, mut i: usize) {
    loop {
        let (l, r, mut max) = (2 * i + 1, 2 * i + 2, i);

        if l < n && nums[l] > nums[max] {
            max = l;
        }
        if r < n && nums[r] > nums[max] {
            max = r;
        }
        if max == i {
            break;
        }
        (nums[max], nums[i]) = (nums[i], nums[max]);
        i = max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let nums = vec![6, 4, 2, 9, 4, 8, 1, 3, 7, 2, 3, 5];

        let nums = heap_sort(nums);

        assert_eq!(nums, vec![1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9]);

        let nums = vec![];
        let nums = heap_sort(nums);
        assert_eq!(nums, vec![]);
    }
}
