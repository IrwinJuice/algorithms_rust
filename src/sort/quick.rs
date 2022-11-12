pub fn sort<T: Ord>(arr: &mut [T]) {
    if arr.len() > 1 {
        quick_sort(arr, 0, arr.len() - 1);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], lo: usize, hi: usize) -> usize {
    let mut i = lo;
    let mut j = hi;
    let pivot = hi;
    while i < j {
        while i < j && arr[i] < arr[pivot] {
            i += 1;
        }

        while i < j && arr[pivot] < arr[j] {
            j -= 1;
        }

        if arr[i] > arr[j] {
            arr.swap(i, j);
        }
    }
    arr.swap(i, pivot);
    i
}

fn quick_sort<T: Ord>(arr: &mut [T], lo: usize, hi: usize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        quick_sort(arr, lo, p - 1);
        quick_sort(arr, p + 1, hi);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let mut res = vec![
            10, -8, 4, -3, -1, 9, 2, -7, -5, 0, -6, -10, 8, -4, 3, 1, -9, -2, 7, 5, 6,
        ];
        sort(&mut res);
        assert_eq!(
            res,
            vec![-10, -9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }

    #[test]
    fn basic_string() {
        let mut res = vec!["a", "bb", "d", "cc"];
        sort(&mut res);
        assert_eq!(res, vec!["a", "bb", "cc", "d"]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<u8>::new();
        sort(&mut res);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        sort(&mut res);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        sort(&mut res);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}
