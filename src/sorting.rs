
/*
reverse list
bubble sort
quick sort
selection sort
merge sort
radix sort
 */
mod sorting {
    // Time: O(n) Space: O(1)
    fn reverse(arr: &mut [i32]) {
        let mut i = 0;
        let mut j = arr.len() -1;
        while i < j {
            let temp = arr[i];
            arr[i] = arr[j];
            arr[j] = temp;
        } 
    }

    // Time O(n^2) Space: O(1)
    fn bubble(arr: &mut [i32]) {
        let mut swap = false;
        while true {
            swap = false;
            for i in 0..arr.len() - 1 {
                let j = i + 1;
                if arr[i] > arr[j] {
                    let temp = arr[i];
                    arr[i] = arr[j];
                    arr[j] = temp;
                    swap = true;
                }
            }
            if !swap {
                break;
            }
        }
    }

    #[cfg(test)]
    mod sorting_test {
        use super::*;

        #[test]
        fn test_reverse() {
            let mut arr = [1, 2, 3, 4, 5];
            reverse(&mut arr);
            assert_eq!(arr, [5, 4, 3, 2, 1]);
        }

        #[test]
        fn test_bubble() {
            let mut arr = [5, 4, 3, 2, 1];
            bubble(&mut arr);
            assert_eq!(arr, [1, 2, 3, 4, 5]);
        }
    }
}
