pub fn counting_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let mut max = None;

    for &i in &nums {
        if max.is_none() || i > max.unwrap() as i32 {
            max = Some(i as usize);
        }
    }

    if let Some(max) = max {
        let mut counter = vec![0; max + 1];
        for &i in &nums {
            counter[i as usize] += 1;
        }

        for i in 0..max {
            counter[i + 1] += counter[i];
        }

        let mut result = vec![0; nums.len()];
        for &num in nums.iter().rev() {
            result[counter[num as usize] as usize - 1] = num;
            counter[num as usize] -= 1;
        }

        for (i, n) in result.into_iter().enumerate() {
            nums[i] = n;
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

        let nums = counting_sort(nums);

        assert_eq!(nums, vec![1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9]);

        let nums = vec![];
        let nums = counting_sort(nums);
        assert_eq!(nums, vec![]);
    }
}
