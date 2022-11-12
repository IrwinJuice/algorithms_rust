/// Heap is a complite binary tree
/// Node is at index i
/// left child is at 2 * i
/// right child is at 2 * i + 1
/// parent is at i / 2
///

pub fn heap_sort<T: PartialOrd>(arr: &mut [T]) {
    build_max_heap(arr);
    let mut n = arr.len() - 1;

    while n != 0 {
        arr.swap(0, n);
        n -= 1;
        heapify(arr, 0);
    }
}

pub fn build_max_heap<T: PartialOrd>(arr: &mut [T]) {
    let mut n = arr.len() - 1;

    while n / 2 != 0 {
        heapify(arr, n);
        n -= 1;
    }
}

pub fn heapify<T: PartialOrd>(arr: &mut [T], i: usize) {
    let mut max: usize;
    let n = arr.len() - 1;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left <= n && arr[left] > arr[i] {
        max = left;
    } else {
        max = i;
    }

    if right <= n && arr[right] > arr[i] {
        max = right;
    }

    if max != i {
        arr.swap(i, max);
        heapify(arr, max);
    }
}
