use std::time::{ Instant, Duration };
use rand::prelude::*;
use super::IsSorted;

/// A trait providing the bogosort method.
pub trait Bogosort<T: PartialEq + PartialOrd + Clone + Copy> {
    /// The bogosort algorithm.
    ///
    /// Sorts the `Vec` it is called on -- or dies trying.
    fn bogosort(&mut self);

    /// The bogosort algorithm but timed.
    ///
    /// Sorts the `Vec` it is called on and returns the `Duration` of the process -- or dies trying.
    fn bogosort_timed(&mut self) -> Duration;

    /// The bogosort algorithm but stepped.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process 
    /// -- or dies trying.
    fn bogosort_stepped(&mut self) -> Vec<Vec<T>>;

    /// The bogosort algorithm but stepped _and_ timed.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process,
    /// including the `Duration` of the entire process -- or dies trying.
    fn bogosort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration);
}

/// The trait implementation of the bogosort algorithm.
impl<T> Bogosort<T> for Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    fn bogosort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        let mut rng = rand::thread_rng();
        while !self.is_sorted() {
            self.shuffle(&mut rng);
        }
    }

    fn bogosort_timed(&mut self) -> Duration {
        let time = Instant::now();

        if self.len() <= 1 {
            return time.elapsed();
        }

        let mut rng = rand::thread_rng();
        while !self.is_sorted() {
            self.shuffle(&mut rng);
        }

        return time.elapsed();
    }

    fn bogosort_stepped(&mut self) -> Vec<Vec<T>> {
        let mut steps = vec![self.clone()];

        if self.len() <= 1 {
            return steps;
        }

        let mut rng = rand::thread_rng();
        while !self.is_sorted() {
            self.shuffle(&mut rng);
            steps.push(self.clone());
        }

        return steps;
    }

    fn bogosort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration) {
        let time = Instant::now();

        let mut steps = vec![self.clone()];

        if self.len() <= 1 {
            return (steps, time.elapsed());
        }

        let mut rng = rand::thread_rng();
        while !self.is_sorted() {
            self.shuffle(&mut rng);
            steps.push(self.clone());
        }

        (steps, time.elapsed())
    }
}

/// The bogosort algorithm.
///
/// Sorts the given `Vec` and returns the result -- or dies trying.
pub fn bogosort<T>(mut arr: Vec<T>) -> Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    if arr.len() <= 1 {
        return arr;
    }

    let mut rng = rand::thread_rng();
    while !arr.is_sorted() {
        arr.shuffle(&mut rng);
    }

    return arr;
}

/// The bogosort algorithm but timed.
///
/// Sorts the given `Vec` and returns the result and the `Duration` of the process -- or dies trying.
pub fn bogosort_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    if arr.len() <= 1 {
        return (arr, time.elapsed());
    }

    let mut rng = rand::thread_rng();
    while !arr.is_sorted() {
        arr.shuffle(&mut rng);
    }

    (arr, time.elapsed())
}

/// The bogosort algorithm but stepped.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing the steps of the process --
/// or dies trying.
pub fn bogosort_stepped<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>) 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let mut steps = vec![arr.clone()];

    if arr.len() <= 1 {
        return (arr, steps);
    }

    let mut rng = rand::thread_rng();
    while !arr.is_sorted() {
        arr.shuffle(&mut rng);
        steps.push(arr.clone());
    }

    (arr, steps)
}

/// The bogosort algorithm but stepped _and_ timed.
///
/// Sorts the given `Vec` and returns the result and a `Vec` containing the steps of the process,
/// including the `Duration` of the entire process -- or dies trying.
pub fn bogosort_stepped_and_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>, Duration)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    let mut steps = vec![arr.clone()];

    if arr.len() <= 1 {
        return (arr, steps, time.elapsed());
    }

    let mut rng = rand::thread_rng();
    while !arr.is_sorted() {
        arr.shuffle(&mut rng);
        steps.push(arr.clone());
    }

    (arr, steps, time.elapsed())
}
