use std::time::{ Instant, Duration };

/// The selection sort algorithm.
///
/// Sorts the given `Vec` and returns the result.
pub fn selection_sort<T>(mut arr: Vec<T>) -> Vec<T>
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    if arr.len() <= 1 {
        return arr;
    }

    let mut sorted = vec![];

    while arr.len() > 0 {
        let mut min_idx = 0;
        for i in 0..arr.len() {
            if arr[min_idx] > arr[i] {
                min_idx = i;
            }
        }
        sorted.push(arr.remove(min_idx));
    }

    return sorted;
}

/// The selection sort algorithm but timed.
///
/// Sorts the given `Vec` and returns the result and the `Duration` of the entire process.
pub fn selection_sort_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    if arr.len() <= 1 {
        return (arr, time.elapsed());
    }

    let mut sorted = vec![];

    while arr.len() > 0 {
        let mut min_idx = 0;
        for i in 0..arr.len() {
            if arr[min_idx] > arr[i] {
                min_idx = i;
            }
        }
        sorted.push(arr.remove(min_idx));
    }

    (sorted, time.elapsed())
}

/// The selection sort algorithm but stepped.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing the steps of the 
/// process as a tuple of the unsorted and sorted array.
pub fn selection_sort_stepped<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<(Vec<T>, Vec<T>)>)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let mut steps = vec![(arr.clone(), vec![])];

    if arr.len() <= 1 {
        return (arr, steps);
    }

    let mut sorted = vec![];

    while arr.len() > 0 {
        let mut min_idx = 0;
        for i in 0..arr.len() {
            if arr[min_idx] > arr[i] {
                min_idx = i;
            }
        }

        sorted.push(arr.remove(min_idx));
        steps.push((arr.clone(), sorted.clone()));
    }

    (sorted, steps)
}

/// The selection sort algorithm but stepped _and_ timed.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing the steps of the 
/// process as a tuple of the unsorted and sorted array, including the `Duration` of the 
/// entire process.
pub fn selection_sort_stepped_and_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<(Vec<T>, Vec<T>)>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    let mut steps = vec![(arr.clone(), vec![])];

    if arr.len() <= 1 {
        return (arr, steps, time.elapsed());
    }

    let mut sorted = vec![];

    while arr.len() > 0 {
        let mut min_idx = 0;
        for i in 0..arr.len() {
            if arr[min_idx] > arr[i] {
                min_idx = i;
            }
        }

        sorted.push(arr.remove(min_idx));
        steps.push((arr.clone(), sorted.clone()));
    }

    (sorted, steps, time.elapsed())
}

