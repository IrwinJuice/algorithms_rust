use std::collections::HashMap;

// Given an integer array nums and an integer k,
// return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.
pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut seen = HashMap::with_capacity(nums.len());
    for (i, n) in nums.into_iter().enumerate() {
        let j = seen.entry(n).or_insert(i);
        if *j != i && (i - *j) <= k as usize {
            return true;
        } else {
            *j = i;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        assert!(contains_nearby_duplicate(nums, k))
    }

    #[test]
    fn test_case_2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        assert!(contains_nearby_duplicate(nums, k))
    }

    #[test]
    fn test_case_3() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        assert!(!contains_nearby_duplicate(nums, k))
    }
}
