use std::time::{ Instant, Duration };

/// A trait providing the bubble sort method.
pub trait BubbleSort<T: PartialEq + PartialOrd + Clone + Copy> {
    /// The bubble sort algorithm.
    ///
    /// Sorts the `Vec` it is called on.
    fn bubble_sort(&mut self);

    /// The bubble sort algorithm but timed.
    ///
    /// Sorts the `Vec` it is called on and returns the `Duration` of the process.
    fn bubble_sort_timed(&mut self) -> Duration;

    /// The bubble sort algorithm but stepped.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process.
    fn bubble_sort_stepped(&mut self) -> Vec<Vec<T>>;

    /// The bubble sort algorithm but stepped _and_ timed.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process,
    /// including the `Duration` of the entire process.
    fn bubble_sort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration);
}

/// The trait implementation of the bubble sort algorithm.
impl<T> BubbleSort<T> for Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    fn bubble_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        let mut is_sorted = false;
        while !is_sorted {
            is_sorted = true;
            for i in 1..self.len() {
                if self[i] < self[i-1] {
                    is_sorted = false;
                    self.swap(i, i-1);
                }
            }
        }
    }

    fn bubble_sort_timed(&mut self) -> Duration {
        let time = Instant::now();

        if self.len() <= 1 {
            return time.elapsed();
        }

        let mut is_sorted = false;
        while !is_sorted {
            is_sorted = true;
            for i in 1..self.len() {
                if self[i] < self[i-1] {
                    is_sorted = false;
                    self.swap(i, i-1);
                }
            }
        }

        return time.elapsed();
    }

    fn bubble_sort_stepped(&mut self) -> Vec<Vec<T>> {
        let mut steps = vec![self.clone()];

        if self.len() <= 1 {
            return steps;
        }

        let mut is_sorted = false;
        while !is_sorted {
            is_sorted = true;
            for i in 1..self.len() {
                if self[i] < self[i-1] {
                    is_sorted = false;
                    self.swap(i, i-1);
                    steps.push(self.clone());
                }
            }
        }

        return steps;
    }

    fn bubble_sort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration) {
        let time = Instant::now();

        let mut steps = vec![self.clone()];

        if self.len() <= 1 {
            return (steps, time.elapsed());
        }

        let mut is_sorted = false;
        while !is_sorted {
            is_sorted = true;
            for i in 1..self.len() {
                if self[i] < self[i-1] {
                    is_sorted = false;
                    self.swap(i, i-1);
                    steps.push(self.clone());
                }
            }
        }

        (steps, time.elapsed())
    }
}

/// The bubble sort algorithm.
///
/// Sorts the given `Vec` and returns the result.
pub fn bubble_sort<T>(mut arr: Vec<T>) -> Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    if arr.len() <= 1 {
        return arr;
    }

    let mut is_sorted = false;
    while !is_sorted {
        is_sorted = true;
        for i in 1..arr.len() {
            if arr[i] < arr[i-1] {
                is_sorted = false;
                arr.swap(i, i-1);
            }
        }
    }

    return arr;
}

/// The bubble sort algorithm but timed.
///
/// Sorts the given `Vec` and returns the result and the `Duration` of the process.
pub fn bubble_sort_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    if arr.len() <= 1 {
        return (arr, time.elapsed());
    }

    let mut is_sorted = false;
    while !is_sorted {
        is_sorted = true;
        for i in 1..arr.len() {
            if arr[i] < arr[i-1] {
                is_sorted = false;
                arr.swap(i, i-1);
            }
        }
    }

    (arr, time.elapsed())
}

/// The bubble sort algorithm but stepped.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing each step of the process.
pub fn bubble_sort_stepped<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let mut steps = vec![arr.clone()];

    if arr.len() <= 1 {
        return (arr, steps);
    }

    let mut is_sorted = false;
    while !is_sorted {
        is_sorted = true;
        for i in 1..arr.len() {
            if arr[i] < arr[i-1] {
                is_sorted = false;
                arr.swap(i, i-1);
                steps.push(arr.clone());
            }
        }
    }

    (arr, steps)
}

/// The bubble sort algorithm but stepped _and_ timed.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing each step of the process.
pub fn bubble_sort_stepped_and_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    let mut steps = vec![arr.clone()];

    if arr.len() <= 1 {
        return (arr, steps, time.elapsed());
    }

    let mut is_sorted = false;
    while !is_sorted {
        is_sorted = true;
        for i in 1..arr.len() {
            if arr[i] < arr[i-1] {
                is_sorted = false;
                arr.swap(i, i-1);
                steps.push(arr.clone());
            }
        }
    }

    (arr, steps, time.elapsed())
}

