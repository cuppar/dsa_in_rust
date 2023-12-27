// input range [0, 1)
pub fn bucket_sort(mut nums: Vec<f64>) -> Vec<f64> {
    let k = nums.len() / 2;

    let mut buckets = vec![vec![]; k];
    for &num in &nums {
        let i = (num * k as f64) as usize;
        buckets[i].push(num);
    }

    for bucket in &mut buckets {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let mut i = 0;
    for bucket in &mut buckets {
        for &mut num in bucket {
            nums[i] = num;
            i += 1;
        }
    }

    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let nums = vec![0.6, 0.4, 0.2, 0.9, 0.4, 0.8, 0.1, 0.3, 0.7, 0.2, 0.3, 0.5];

        let nums = bucket_sort(nums);

        assert_eq!(
            nums,
            vec![0.1, 0.2, 0.2, 0.3, 0.3, 0.4, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9]
        );

        let nums = vec![];
        let nums = bucket_sort(nums);
        assert_eq!(nums, vec![]);
    }
}
