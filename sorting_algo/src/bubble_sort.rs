// Bubble sort is a simple sorting algorithm. This sorting algorithm is comparison-based algorithm in which each pair of adjacent elements is compared and the elements are swapped if they are not in order. This algorithm is not suitable for large data sets as its average and worst case complexity are of
// Ο(n2) where n is the number of items.

#[allow(dead_code)]
fn bubble_sort(arr: &mut Vec<u32>) -> Vec<u32> {
    if arr.is_empty() {
        return vec![];
    }
    for _ in 0..arr.len() - 1 {
        let mut swapped = false;
        for j in 0..arr.len() - 1 {
            if arr[j] > arr[j + 1] {
                /*
                let temp_val = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp_val;
                */
                arr.swap(j, j + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }

    arr.to_vec()
}

#[cfg(test)]
mod tests {
    use super::bubble_sort;

    #[test]
    fn empty_array() {
        // Arrange
        let mut arr: Vec<u32> = Vec::new();

        // Execute
        let res = bubble_sort(&mut arr);

        // Assert
        assert!(res.is_empty());
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_small_array() {
        // Arrange
        let mut arr = vec![2, 9, 7, 3, 5, 1];

        // Execute
        let res = bubble_sort(&mut arr);

        // Assert
        assert_eq!(res, vec![1, 2, 3, 5, 7, 9]);
    }
    #[test]
    fn test_med_array() {
        // Arrange
        let mut arr = vec![2_287, 9_981_823, 7_123, 3_000, 500, 10];

        // Execute
        let res = bubble_sort(&mut arr);

        // Assert
        assert_eq!(res, vec![10, 500, 2_287, 3_000, 7_123, 9_981_823]);
    }

    #[test]
    fn test_large_array() {
        // Arrange
        let mut arr = vec![2_287, 9_981_823, 7_123, 3_000, 500, 10];

        // Execute
        let res = bubble_sort(&mut arr);

        // Assert
        assert_eq!(res, vec![10, 500, 2_287, 3_000, 7_123, 9_981_823]);
    }

    #[test]
    fn test_sorted_array() {
        // Arrange
        let mut arr = vec![5, 20, 25, 28, 35, 39];

        // Execute
        let res = bubble_sort(&mut arr);

        // Assert
        assert_eq!(res, vec![5, 20, 25, 28, 35, 39]);
    }
}
