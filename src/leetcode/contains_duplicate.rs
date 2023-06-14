fn contains_duplicate(nums: Vec<i32>) -> bool {
    if nums.len() < 2 {
        return false;
    }

    let mut i = 0;
    let mut j = 1;
    while i < nums.len() {
        while j < nums.len() {
            if nums[i] == nums[j] {
                return true;
            }

            j += 1;
        }
        i += 1;
        j = i + 1;
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![1,2,3,1];
        assert!(contains_duplicate(nums))
    }
    #[test]
    fn test_case_2() {
        let nums = vec![1,2,3,4];
        assert!(!contains_duplicate(nums))
    }
    #[test]
    fn test_case_3() {
        let nums = vec![2,14,18,22,22];
        assert!(contains_duplicate(nums))
    }
}