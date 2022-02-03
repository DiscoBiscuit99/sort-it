use std::time::{ Instant, Duration };

/// A trait providing the slowsort algorithm.
pub trait Slowsort<T: PartialEq + PartialOrd + Clone + Copy> {
    /// The slowsort algorithm.
    ///
    /// Sorts the `Vec` it is called on.
    fn slowsort(&mut self);

    /// The slowsort algorithm but timed.
    ///
    /// Sorts the `Vec` it is called on and returns the `Duration` of the process.
    fn slowsort_timed(&mut self) -> Duration;

    /// The slowsort algorithm but stepped.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process.
    fn slowsort_stepped(&mut self) -> Vec<Vec<T>>;

    /// The slowsort algorithm but stepped _and_ timed.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process,
    /// including the `Duration` of the entire process.
    fn slowsort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration);
}

/// The trait implementation of the slowsort algorithm.
impl<T> Slowsort<T> for Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    fn slowsort(&mut self) {
        // NOTE: this may be too serious of an optimization 
        //       for the kind of algorithm this is (and might 
        //       even defeat the purpose).
        if self.len() <= 1 {
            return;
        }

        let length = self.len();
        slowsort_rec(self, 0, length - 1);
    }

    fn slowsort_timed(&mut self) -> Duration {
        let time = Instant::now();

        // NOTE: this may be too serious of an optimization 
        //       for the kind of algorithm this is (and might 
        //       even defeat the purpose).
        if self.len() <= 1 {
            return time.elapsed();
        }

        let length = self.len();
        slowsort_rec(self, 0, length - 1);

        return time.elapsed();
    }

    fn slowsort_stepped(&mut self) -> Vec<Vec<T>> {
        let mut steps = vec![self.clone()];

        // NOTE: this may be too serious of an optimization 
        //       for the kind of algorithm this is (and might 
        //       even defeat the purpose).
        if self.len() <= 1 {
            return steps;
        }

        let length = self.len();
        slowsort_rec_stepped(self, 0, length - 1, &mut steps);

        return steps;
    }

    fn slowsort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration) {
        let time = Instant::now();

        let mut steps = vec![self.clone()];

        // NOTE: this may be too serious of an optimization 
        //       for the kind of algorithm this is (and might 
        //       even defeat the purpose).
        if self.len() <= 1 {
            return (steps, time.elapsed());
        }

        let length = self.len();
        slowsort_rec_stepped(self, 0, length - 1, &mut steps);

        (steps, time.elapsed())
    }
}

/// The slowsort algorithm.
///
/// Sorts the given `Vec` and returns the result.
pub fn slowsort<T>(mut arr: Vec<T>) -> Vec<T>
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    // NOTE: this may be too serious of an optimization 
    //       for the kind of algorithm this is (and might 
    //       even defeat the purpose).
    if arr.len() <= 1 {
        return arr;
    }

    let length = arr.len();
    slowsort_rec(&mut arr, 0, length - 1);

    return arr;
}

/// The slowsort algorithm but timed.
///
/// Sorts the given `Vec` and returns the result and the `Duration` of the process.
pub fn slowsort_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    // NOTE: this may be too serious of an optimization 
    //       for the kind of algorithm this is (and might 
    //       even defeat the purpose).
    if arr.len() <= 1 {
        return (arr, time.elapsed());
    }

    let length = arr.len();
    slowsort_rec(&mut arr, 0, length - 1);

    (arr, time.elapsed())
}

/// The slowsort algorithm but stepped.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing each step of the process.
pub fn slowsort_stepped<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let mut steps = vec![arr.clone()];

    // NOTE: this may be too serious of an optimization 
    //       for the kind of algorithm this is (and might 
    //       even defeat the purpose).
    if arr.len() <= 1 {
        return (arr, steps);
    }

    let length = arr.len();
    slowsort_rec_stepped(&mut arr, 0, length - 1, &mut steps);

    (arr, steps)
}

/// The slowsort algorithm but stepped _and_ timed.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing each step of the process,
/// including the `Duration` of the entire process.
pub fn slowsort_stepped_and_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>, Duration) 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    let mut steps = vec![arr.clone()];

    // NOTE: this may be too serious of an optimization 
    //       for the kind of algorithm this is (and might 
    //       even defeat the purpose).
    if arr.len() <= 1 {
        return (arr, steps, time.elapsed());
    }

    let length = arr.len();
    slowsort_rec_stepped(&mut arr, 0, length - 1, &mut steps);

    (arr, steps, time.elapsed())
}

/// Auxiliary function.
fn slowsort_rec<T>(arr: &mut Vec<T>, i: usize, j: usize) 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    if i >= j {
        return;
    }

    let m = (i + j) / 2;

    slowsort_rec(arr, i, m);
    slowsort_rec(arr, m+1, j);

    if arr[m] > arr[j] {
        arr.swap(m, j);
    }

    slowsort_rec(arr, i, j-1);
}

/// Auxiliary function (with stepped support).
fn slowsort_rec_stepped<T>(arr: &mut Vec<T>, i: usize, j: usize, steps: &mut Vec<Vec<T>>)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    if i >= j {
        return;
    }

    let m = (i + j) / 2;

    slowsort_rec(arr, i, m);
    slowsort_rec(arr, m+1, j);

    if arr[m] > arr[j] {
        arr.swap(m, j);
        steps.push(arr.clone());
    }

    slowsort_rec(arr, i, j-1);
}
