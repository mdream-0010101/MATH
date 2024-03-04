fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot_index = partition(arr);
    quick_sort(&mut arr[0..pivot_index]);
    quick_sort(&mut arr[pivot_index + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() / 2;

    arr.swap(pivot_index, arr.len() - 1);
    let mut i = 0;
    for j in 0..arr.len() - 1 {
        if arr[j] <= arr[arr.len() - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, arr.len() - 1);
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr1 = [5, 2, 3, 1, 4];
        quick_sort(&mut arr1);
        assert_eq!(arr, [1, 2, 3, 4, 5]);

        let mut arr2 = [5, 5, 5, 5, 5];
        quick_sort(&mut arr2);
        assert_eq!(arr2m [5, 5, 5, 5, 5]);

        let mut arr3: [char; 5] = ['e', 'a', 'c', 'd', 'b'];
        quick_sort(&mut arr3);
        assert_eq!(arr3, ['a', 'b', 'c', 'd', 'e']);
    }
}
