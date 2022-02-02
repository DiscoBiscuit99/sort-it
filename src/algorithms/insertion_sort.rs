use std::time::{ Instant, Duration };

/// A trait providing the insertion sort method.
pub trait InsertionSort<T: PartialEq + PartialOrd + Clone + Copy> {
    /// The insertion sort algorithm.
    ///
    /// Sorts the `Vec` it is called on.
    fn insertion_sort(&mut self);

    /// The insertion sort algorithm but timed.
    ///
    /// Sorts the `Vec` it is called on and returns the `Duration` of the process.
    fn insertion_sort_timed(&mut self) -> Duration;

    /// The insertion sort algorithm but stepped.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process.
    fn insertion_sort_stepped(&mut self) -> Vec<Vec<T>>;

    /// The insertion sort algorithm but stepped _and_ timed.
    ///
    /// Sorts the `Vec` it is called on and returns the `Duration` of the process, including the
    /// `Duration` of the entire process.
    fn insertion_sort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration);
}

/// The trait implementation of the insertion sort algorithm.
impl<T> InsertionSort<T> for Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    fn insertion_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        for i in 0..self.len() {
            let mut j = i;
            while j > 0 && self[j] < self[j-1] {
                self.swap(j, j-1);
                j -= 1;
            }
        }
    }

    fn insertion_sort_timed(&mut self) -> Duration {
        let time = Instant::now();

        if self.len() <= 1 {
            return time.elapsed();
        }

        for i in 0..self.len() {
            let mut j = i;
            while j > 0 && self[j] < self[j-1] {
                self.swap(j, j-1);
                j -= 1;
            }
        }

        return time.elapsed();
    }

    fn insertion_sort_stepped(&mut self) -> Vec<Vec<T>> {
        let mut steps = vec![self.clone()];

        if self.len() <= 1 {
            return steps;
        }

        for i in 0..self.len() {
            let mut j = i;
            while j > 0 && self[j] < self[j-1] {
                self.swap(j, j-1);
                steps.push(self.clone());
                j -= 1;
            }
        }

        return steps;
    }

    fn insertion_sort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration) {
        let time = Instant::now();

        let mut steps = vec![self.clone()];

        if self.len() <= 1 {
            return (steps, time.elapsed());
        }

        for i in 0..self.len() {
            let mut j = i;
            while j > 0 && self[j] < self[j-1] {
                self.swap(j, j-1);
                steps.push(self.clone());
                j -= 1;
            }
        }

        (steps, time.elapsed())
    }
}

/// The insertion sort algorithm.
///
/// Sorts the given `Vec` and returns the result.
pub fn insertion_sort<T>(mut arr: Vec<T>) -> Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    if arr.len() <= 1 {
        return arr;
    }

    for i in 0..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1] {
            arr.swap(j, j-1);
            j -= 1;
        }
    }

    return arr;
}

/// The insertion sort algorithm but timed.
///
/// Sorts the given `Vec` and returns the result and the `Duration` of the entire process.
pub fn insertion_sort_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    if arr.len() <= 1 {
        return (arr, time.elapsed());
    }

    for i in 0..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1] {
            arr.swap(j, j-1);
            j -= 1;
        }
    }

    (arr, time.elapsed())
}

/// The insertion sort algorithm but stepped.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing all steps of the process.
pub fn insertion_sort_stepped<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let mut steps = vec![arr.clone()];

    if arr.len() <= 1 {
        return (arr, steps);
    }

    for i in 0..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1] {
            arr.swap(j, j-1);
            steps.push(arr.clone());
            j -= 1;
        }
    }

    (arr, steps)
}

/// The insertion sort algorithm but stepped _and_ timed.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing all steps of the process,
/// including the `Duration` of the entire process.
pub fn insertion_sort_stepped_and_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    let mut steps = vec![arr.clone()];

    if arr.len() <= 1 {
        return (arr, steps, time.elapsed());
    }

    for i in 0..arr.len() {
        let mut j = i;
        while j > 0 && arr[j] < arr[j-1] {
            arr.swap(j, j-1);
            steps.push(arr.clone());
            j -= 1;
        }
    }

    (arr, steps, time.elapsed())
}

