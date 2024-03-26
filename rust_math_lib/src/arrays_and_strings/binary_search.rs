fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search_found() {
        let arr = [1, 3, 5, 7, 9, 11, 13, 15, 17];
        assert_eq!(binary_search(&arr, 7), Some(3));
    }

    #[test]
    fn test_binary_search_not_found() {
        let arr = [1, 3, 5, 7, 9, 11, 13, 15, 17];
        assert_eq!(binary_search(&arr, 8), None);
    }

    #[test]
    fn test_binary_search_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, 5), None)
    }
}
