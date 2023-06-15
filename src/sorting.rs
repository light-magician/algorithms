
/*
Sorting algorithms
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
            i += 1;
            j -= 1;
        } 
    }

    // Time O(n^2) Space: O(1)
    fn bubble(arr: &mut [i32]) {
        loop {
            let mut swap = false;
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

    fn quick(arr: &mut [i32]) {
        let len = arr.len();
        if len <= 1 {
            return;
        }
        let pivot_index = partition(arr);
        // quicksort left of pivot
        quick(&mut arr[0..pivot_index]);
        // quicksort right of pivot
        quick(&mut arr[pivot_index + 1..len]);

    }

    fn partition(arr: &mut [i32]) -> usize {
        let len = arr.len();
        let pivot_index = len /2;
        let pivot = arr[pivot_index];
        arr.swap(pivot_index, len - 1);

        let mut i = 0;
        for j in 0..len - 1 {
            if arr[j] <= pivot {
                arr.swap(i, j);
                i += 1;
            }
        }

        arr.swap(i, len - 1);
        i
    }

    #[cfg(test)]
    mod sorting_test {
        use super::*;
        use std::time::Instant;

        #[test]
        fn test_reverse() {
            let mut arr = [1, 2, 3, 4, 5];
            let start = Instant::now();
            reverse(&mut arr);
            report_timing(start, "reverse list");
            assert_eq!(arr, [5, 4, 3, 2, 1]);
        }

        #[test]
        fn test_bubble() {
            let mut arr = [5, 4, 3, 2, 1];
            let start = Instant::now();
            bubble(&mut arr);
            report_timing(start, "bubblesort");
            assert_eq!(arr, [1, 2, 3, 4, 5]);
        }

        #[test]
        fn test_quick() {
            let mut arr = [5, 4, 3, 2, 1];
            let start = Instant::now();
            quick(&mut arr);
            report_timing(start, "quicksort");
            assert_eq!(arr, [1, 2, 3, 4, 5]);
        }

        fn report_timing(start: Instant, name: &str) {
            let elapsed = start.elapsed();
            println!("{} execution time: {:?}", name, elapsed);
        }
    }
}
