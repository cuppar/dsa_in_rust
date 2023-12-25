pub fn select_sort(mut src: Vec<i32>) -> Vec<i32> {
    let n = src.len();
    if n < 2 {
        return src;
    }

    for i in 0..n - 1 {
        let mut k = i;
        for j in i + 1..n {
            if src[j] < src[k] {
                k = j;
            }
        }

        swap(&mut src, i, k);
    }
    src
}

fn swap(src: &mut Vec<i32>, i: usize, j: usize) {
    let n = src.len();
    if i >= n || j >= n {
        panic!("our of index");
    }
    if i != j {
        let (max, min) = (i.max(j), i.min(j));
        let (left, right) = src.split_at_mut(max);
        std::mem::swap(&mut left[min], &mut right[0]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let src = vec![6, 4, 2, 9, 4, 8, 1, 3, 7, 2, 3, 5];

        let src = select_sort(src);

        assert_eq!(src, vec![1, 2, 2, 3, 3, 4, 4, 5, 6, 7, 8, 9]);

        let src = vec![];
        let src = select_sort(src);
        assert_eq!(src, vec![]);
    }
}
