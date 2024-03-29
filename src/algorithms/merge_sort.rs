use std::time::{ Instant, Duration };

/// A trait providing the merge sort method.
pub trait MergeSort<T: PartialEq + PartialOrd + Clone + Copy> {
    /// The merge sort algorithm.
    /// 
    /// Sorts the `Vec` it is called on.
    fn merge_sort(&mut self);

    /// The merge sort algorithm but timed.
    /// 
    /// Sorts the `Vec` it is called on and returns the `Duration` of the process.
    fn merge_sort_timed(&mut self) -> Duration;

    /// The merge sort algorithm but stepped.
    /// 
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process.
    fn merge_sort_stepped(&mut self) -> Vec<Vec<T>>;

    /// The merge sort algorithm but stepped _and_ timed.
    /// 
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process, 
    /// including the `Duration` of the entire process.
    fn merge_sort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration);
}

/// The trait implementation of the merge sort algorithm.
impl<T> MergeSort<T> for Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    fn merge_sort(&mut self) {
        // If the array only contains one element, it's sorted by default.
        if self.len() <= 1 {
            return;
        }

        // Obtain the right- and left-hand-sides.
        let rhs = &self[..self.len()/2];
        let lhs = &self[self.len()/2..];

        *self = merge_rec(rhs.to_vec(), lhs.to_vec());
    }

    fn merge_sort_timed(&mut self) -> Duration {
        let time = Instant::now();

        // If the array only contains one element, it's sorted by default.
        if self.len() <= 1 {
            return time.elapsed();
        }

        // Obtain the right- and left-hand-sides.
        let rhs = &self[..self.len()/2];
        let lhs = &self[self.len()/2..];

        *self = merge_rec(rhs.to_vec(), lhs.to_vec());

        return time.elapsed();
    }

    fn merge_sort_stepped(&mut self) -> Vec<Vec<T>> {
        let mut steps = vec![self.clone()];

        // If the array only contains one element, it's sorted by default.
        if self.len() <= 1 {
            return steps;
        }

        // Obtain the right- and left-hand-sides.
        let rhs = &self[..self.len()/2];
        let lhs = &self[self.len()/2..];

        *self = merge_rec_stepped(rhs.to_vec(), lhs.to_vec(), &mut steps);

        return steps;
    }

    fn merge_sort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration) {
        let time = Instant::now();

        let mut steps = vec![self.clone()];

        // If the array only contains one element, it's sorted by default.
        if self.len() <= 1 {
            return (steps, time.elapsed());
        }

        // Obtain the right- and left-hand-sides.
        let rhs = &self[..self.len()/2];
        let lhs = &self[self.len()/2..];

        *self = merge_rec_stepped(rhs.to_vec(), lhs.to_vec(), &mut steps);

        (steps, time.elapsed())
    }
}

/// The merge sort algorithm.
/// 
/// Sorts the given `Vec` and returns the result.
pub fn merge_sort<T>(arr: Vec<T>) -> Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    // If the array only contains one element, it's sorted by default.
    if arr.len() <= 1 {
        return arr;
    }

    // Obtain the right- and left-hand-sides.
    let rhs = &arr[..arr.len()/2];
    let lhs = &arr[arr.len()/2..];

    merge_rec(rhs.to_vec(), lhs.to_vec())
}

/// The merge sort algorithm but timed.
///
/// Sorts the given `Vec` and returns the result and the `Duration` of the process.
pub fn merge_sort_timed<T>(arr: Vec<T>) -> (Vec<T>, Duration) 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    // If the array only contains one element, it's sorted by default.
    if arr.len() <= 1 {
        return (arr, time.elapsed());
    }

    // Obtain the right- and left-hand-sides.
    let rhs = &arr[..arr.len()/2];
    let lhs = &arr[arr.len()/2..];

    (merge_rec(rhs.to_vec(), lhs.to_vec()), time.elapsed())
}

/// The merge sort algorithm but stepped.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing all the steps of the entire 
/// process.
pub fn merge_sort_stepped<T>(arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let mut steps = vec![arr.clone()];

    // If the array only contains one element, it's sorted by default.
    if arr.len() <= 1 {
        return (arr, steps);
    }

    // Obtain the right- and left-hand-sides.
    let rhs = &arr[..arr.len()/2];
    let lhs = &arr[arr.len()/2..];

    let sorted = merge_rec_stepped(rhs.to_vec(), lhs.to_vec(), &mut steps);

    (sorted, steps)
}

/// The merge sort algorithm but stepped _and_ timed.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing all the steps of the
/// process, including the `Duration` of the entire process.
pub fn merge_sort_stepped_and_timed<T>(arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>, Duration) 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    let mut steps = vec![arr.clone()];

    // If the array only contains one element, it's sorted by default.
    if arr.len() <= 1 {
        return (arr, steps, time.elapsed());
    }

    // Obtain the right- and left-hand-sides.
    let rhs = &arr[..arr.len()/2];
    let lhs = &arr[arr.len()/2..];

    let sorted = merge_rec_stepped(rhs.to_vec(), lhs.to_vec(), &mut steps);

    (sorted, steps, time.elapsed())
}

/// Auxiliary merge function.
fn merge_rec<T>(mut rhs: Vec<T>, mut lhs: Vec<T>) -> Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let mut sorted = vec![];

    if rhs.len() > 1 {
        let new_rhs = &rhs[..rhs.len()/2];
        let new_lhs = &rhs[rhs.len()/2..];

        rhs = merge_rec(new_rhs.to_vec(), new_lhs.to_vec());
    }
    if lhs.len() > 1 {
        let new_rhs = &lhs[..lhs.len()/2];
        let new_lhs = &lhs[lhs.len()/2..];

        lhs = merge_rec(new_rhs.to_vec(), new_lhs.to_vec());
    }

    let mut i = 0;
    let mut j = 0;

    while i < rhs.len() && j < lhs.len() {
        if rhs[i] <= lhs[j] {
            sorted.push(rhs[i]);
            i += 1;
        } else {
            sorted.push(lhs[j]);
            j += 1;
        }
    }

    if i == rhs.len() {
        for k in j..lhs.len() {
            sorted.push(lhs[k]);
        }
    } else if j == lhs.len() {
        for k in i..rhs.len() {
            sorted.push(rhs[k]);
        }
    }

    return sorted;
}

/// Auxiliary merge function with step support.
fn merge_rec_stepped<T>(mut rhs: Vec<T>, mut lhs: Vec<T>, steps: &mut Vec<Vec<T>>) -> Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let mut sorted = vec![];

    if rhs.len() > 1 {
        let new_rhs = &rhs[..rhs.len()/2];
        let new_lhs = &rhs[rhs.len()/2..];

        rhs = merge_rec_stepped(new_rhs.to_vec(), new_lhs.to_vec(), steps);
    }
    if lhs.len() > 1 {
        let new_rhs = &lhs[..lhs.len()/2];
        let new_lhs = &lhs[lhs.len()/2..];

        lhs = merge_rec_stepped(new_rhs.to_vec(), new_lhs.to_vec(), steps);
    }

    let mut i = 0;
    let mut j = 0;

    while i < rhs.len() && j < lhs.len() {
        if rhs[i] <= lhs[j] {
            sorted.push(rhs[i]);
            i += 1;
        } else {
            sorted.push(lhs[j]);
            j += 1;
        }
    }

    if i == rhs.len() {
        for k in j..lhs.len() {
            sorted.push(lhs[k]);
        }
    } else if j == lhs.len() {
        for k in i..rhs.len() {
            sorted.push(rhs[k]);
        }
    }

    steps.push(sorted.clone());

    return sorted;
}
