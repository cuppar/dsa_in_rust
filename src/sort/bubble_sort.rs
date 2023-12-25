pub fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    for j in (1..n).rev() {
        let mut exchanged = false;
        for i in 0..j {
            if nums[i] > nums[i + 1] {
                swap(&mut nums, i, i + 1);
                exchanged = true;
            }
        }
        if !exchanged {
            break;
        }
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

        let nums = bubble_sort(nums);

        assert_eq!(nums, vec![1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9]);

        let nums = vec![];
        let nums = bubble_sort(nums);
        assert_eq!(nums, vec![]);
    }
}
