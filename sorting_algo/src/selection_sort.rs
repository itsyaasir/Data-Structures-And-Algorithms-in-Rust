// The selection sort algorithm sorts an array by repeatedly finding the minimum element (considering ascending order) from the unsorted part and putting it at the beginning.

#[allow(dead_code)]
fn selection_sort(arr: &mut Vec<u32>) -> Vec<u32> {
    // If Array is empty, we return empty Vec
    if arr.is_empty() {
        return vec![];
    }

    for i in 0..arr.len() - 1 {
        // We are keeping the track of the lowest index
        let mut min_idx = i;
        // We looping the i value through the second loop
        for j in i + 1..arr.len() {
            // If any value is less than our min index, we assign their index as the min index
            if arr[min_idx] > arr[j] {
                min_idx = j;
            }

            if min_idx != i {
                arr.swap(min_idx, i);
            }
        }
    }
    arr.to_vec()
}

#[cfg(test)]
mod tests {
    use super::selection_sort;

    #[test]
    fn empty_array() {
        // Arrange
        let mut arr: Vec<u32> = Vec::new();

        // Execute
        let res = selection_sort(&mut arr);

        // Assert
        assert!(res.is_empty());
        assert_eq!(res.len(), 0);
    }

    #[test]
    fn test_small_array() {
        // Arrange
        let mut arr = vec![2, 9, 7, 3, 8, 1];

        // Execute
        let res = selection_sort(&mut arr);

        // Assert
        assert_eq!(res, vec![1, 2, 3, 7, 8, 9]);
    }
    #[test]
    fn test_med_array() {
        // Arrange
        let mut arr = vec![2_287, 9_981_823, 7_123, 3_000, 500, 10, 6];

        // Execute
        let res = selection_sort(&mut arr);

        // Assert
        assert_eq!(res, vec![6, 10, 500, 2_287, 3_000, 7_123, 9_981_823]);
    }

    #[test]
    fn test_large_array() {
        // Arrange
        let mut arr = vec![2_287, 9_981_823, 7_123, 3_000, 500, 10];

        // Execute
        let res = selection_sort(&mut arr);

        // Assert
        assert_eq!(res, vec![10, 500, 2_287, 3_000, 7_123, 9_981_823]);
    }

    #[test]
    fn test_sorted_array() {
        // Arrange
        let mut arr = vec![5, 20, 25, 28, 35, 39];

        // Execute
        let res = selection_sort(&mut arr);

        // Assert
        assert_eq!(res, vec![5, 20, 25, 28, 35, 39]);
    }
}
