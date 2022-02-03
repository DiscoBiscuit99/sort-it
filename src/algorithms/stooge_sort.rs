use std::time::{ Instant, Duration };

/// A trait providing the stooge sort algorithm.
pub trait StoogeSort<T: PartialEq + PartialOrd + Clone + Copy> {
    /// The stooge sort algorithm.
    ///
    /// Sorts the `Vec` it is called on.
    fn stooge_sort(&mut self);

    /// The stooge sort algorithm but timed.
    ///
    /// Sorts the `Vec` it is called on and returns the `Duration` of the process.
    fn stooge_sort_timed(&mut self) -> Duration;

    /// The stooge sort algorithm but stepped.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process.
    fn stooge_sort_stepped(&mut self) -> Vec<Vec<T>>;

    /// The stooge sort algorithm but stepped _and_ timed.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process,
    /// including the `Duration` of the entire process.
    fn stooge_sort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration);
}

impl<T> StoogeSort<T> for Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    fn stooge_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        let length = self.len();
        stooge_sort_rec(self, 0, length - 1);
    }

    fn stooge_sort_timed(&mut self) -> Duration {
        let time = Instant::now();

        if self.len() <= 1 {
            return time.elapsed();
        }

        let length = self.len();
        stooge_sort_rec(self, 0, length - 1);

        return time.elapsed();
    }

    fn stooge_sort_stepped(&mut self) -> Vec<Vec<T>> {
        let mut steps = vec![self.clone()];

        if self.len() <= 1 {
            return steps;
        }

        let length = self.len();
        stooge_sort_rec_stepped(self, 0, length - 1, &mut steps);

        return steps;
    }

    fn stooge_sort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration) {
        let time = Instant::now();

        let mut steps = vec![self.clone()];

        if self.len() <= 1 {
            return (steps, time.elapsed());
        }

        let length = self.len();
        stooge_sort_rec_stepped(self, 0, length - 1, &mut steps);

        (steps, time.elapsed())
    }
}

/// The stooge sort algorithm.
///
/// Sorts the given `Vec` and returns the result.
pub fn stooge_sort<T>(mut arr: Vec<T>) -> Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    if arr.len() <= 1 {
        return arr;
    }

    let length = arr.len();
    stooge_sort_rec(&mut arr, 0, length - 1);

    return arr;
}

/// The stooge sort algorithm but timed.
///
/// Sorts the given `Vec` and returns the result and the `Duration` of the process.
pub fn stooge_sort_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    if arr.len() <= 1 {
        return (arr, time.elapsed());
    }

    let length = arr.len();
    stooge_sort_rec(&mut arr, 0, length - 1);

    (arr, time.elapsed())
}

/// The stooge sort algorithm but stepped.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing each step of the process.
pub fn stooge_sort_stepped<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let mut steps = vec![arr.clone()];

    if arr.len() <= 1 {
        return (arr, steps);
    }

    let length = arr.len();
    stooge_sort_rec_stepped(&mut arr, 0, length - 1, &mut steps);

    (arr, steps)
}

/// The stooge sort algorithm but stepped _and_ timed.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing each step of the process,
/// including the `Duration` of the entire process.
pub fn stooge_sort_stepped_and_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    let mut steps = vec![arr.clone()];

    if arr.len() <= 1 {
        return (arr, steps, time.elapsed());
    }

    let length = arr.len();
    stooge_sort_rec_stepped(&mut arr, 0, length - 1, &mut steps);

    (arr, steps, time.elapsed())
}

/// Auxiliary function.
fn stooge_sort_rec<T>(arr: &mut Vec<T>, i: usize, j: usize)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    if arr[i] > arr[j] {
        arr.swap(i, j);
    }

    if (j - i + 1) > 2 {
        let t = (j - i + 1) / 3;

        stooge_sort_rec(arr, i, j - t);
        stooge_sort_rec(arr, i + t, j);
        stooge_sort_rec(arr, i, j - t);
    }
}

/// Auxiliary function (but stepped).
fn stooge_sort_rec_stepped<T>(arr: &mut Vec<T>, i: usize, j: usize, steps: &mut Vec<Vec<T>>)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    if arr[i] > arr[j] {
        arr.swap(i, j);
    }

    if (j - i + 1) > 2 {
        let t = (j - i + 1) / 3;

        stooge_sort_rec(arr, i, j - t);
        steps.push(arr.clone());
        stooge_sort_rec(arr, i + t, j);
        steps.push(arr.clone());
        stooge_sort_rec(arr, i, j - t);
        steps.push(arr.clone());
    }
}
