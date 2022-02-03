use std::time::{ Instant, Duration };

/// A trait providing the selection sort algorithm.
pub trait SelectionSort<T: PartialEq + PartialOrd + Clone + Copy> {
    /// The selection sort algorithm.
    ///
    /// Sorts the `Vec` it is called on.
    fn selection_sort(&mut self);

    /// The selection sort algorithm but timed.
    ///
    /// Sorts the `Vec` it is called on and returns the `Duration` of the process.
    fn selection_sort_timed(&mut self) -> Duration;

    /// The selection sort algorithm but stepped.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process.
    fn selection_sort_stepped(&mut self) -> Vec<(Vec<T>, Vec<T>)>;

    /// The selection sort algorithm but stepped _and_ timed.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process,
    /// including the `Duration` of the entire process.
    fn selection_sort_stepped_and_timed(&mut self) -> (Vec<(Vec<T>, Vec<T>)>, Duration);
}

/// The trait implementation of the selection sort algorithm.
impl<T> SelectionSort<T> for Vec<T>
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    fn selection_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        let mut sorted = vec![];

        while self.len() > 0 {
            let mut min_idx = 0;
            for i in 0..self.len() {
                if self[min_idx] > self[i] {
                    min_idx = i;
                }
            }
            sorted.push(self.remove(min_idx));
        }

        *self = sorted;
    }

    fn selection_sort_timed(&mut self) -> Duration {
        let time = Instant::now();

        if self.len() <= 1 {
            return time.elapsed();
        }

        let mut sorted = vec![];

        while self.len() > 0 {
            let mut min_idx = 0;
            for i in 0..self.len() {
                if self[min_idx] > self[i] {
                    min_idx = i;
                }
            }
            sorted.push(self.remove(min_idx));
        }

        *self = sorted;

        return time.elapsed();
    }

    fn selection_sort_stepped(&mut self) -> Vec<(Vec<T>, Vec<T>)> {
        let mut steps = vec![(self.clone(), vec![])];

        if self.len() <= 1 {
            return steps;
        }

        let mut sorted = vec![];

        while self.len() > 0 {
            let mut min_idx = 0;
            for i in 0..self.len() {
                if self[min_idx] > self[i] {
                    min_idx = i;
                }
            }
            sorted.push(self.remove(min_idx));
            steps.push((self.clone(), sorted.clone()));
        }

        *self = sorted;

        return steps;
    }

    fn selection_sort_stepped_and_timed(&mut self) -> (Vec<(Vec<T>, Vec<T>)>, Duration) {
        let time = Instant::now();

        let mut steps = vec![(self.clone(), vec![])];

        if self.len() <= 1 {
            return (steps, time.elapsed());
        }

        let mut sorted = vec![];

        while self.len() > 0 {
            let mut min_idx = 0;
            for i in 0..self.len() {
                if self[min_idx] > self[i] {
                    min_idx = i;
                }
            }
            sorted.push(self.remove(min_idx));
            steps.push((self.clone(), sorted.clone()));
        }

        *self = sorted;

        (steps, time.elapsed())
    }
}

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

