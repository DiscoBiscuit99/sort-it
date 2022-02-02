use std::time::{ Instant, Duration };

/// A trait providing the gnome sort method.
pub trait GnomeSort<T: PartialEq + PartialOrd + Clone + Copy> {
    /// The gnome sort algorithm.
    ///
    /// Sorts the `Vec` it is called on.
    fn gnome_sort(&mut self);

    /// The gnome sort algorithm but timed.
    ///
    /// Sorts the `Vec` it is called on and returns the `Duration` of the process.
    fn gnome_sort_timed(&mut self) -> Duration;

    /// The gnome sort algorithm but stepped.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process.
    fn gnome_sort_stepped(&mut self) -> Vec<Vec<T>>;

    /// The gnome sort algorithm but stepped _and_ timed.
    ///
    /// Sorts the `Vec` it is called on and returns a `Vec` containing each step of the process,
    /// including a `Duration` of the entire process.
    fn gnome_sort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration);
}

/// The trait implementation of the bogosort algorithm.
impl<T> GnomeSort<T> for Vec<T>
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    fn gnome_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        let mut i = 1;

        while i < self.len() {
            if self[i] < self[i-1] {
                let tmp = self[i];
                self[i] = self[i-1];
                self[i-1] = tmp;

                if i > 1 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }
    }

    fn gnome_sort_timed(&mut self) -> Duration {
        let time = Instant::now();

        if self.len() <= 1 {
            return time.elapsed();
        }

        let mut i = 1;

        while i < self.len() {
            if self[i] < self[i-1] {
                let tmp = self[i];
                self[i] = self[i-1];
                self[i-1] = tmp;

                if i > 1 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }

        return time.elapsed();
    }

    fn gnome_sort_stepped(&mut self) -> Vec<Vec<T>> {
        let mut steps = vec![self.clone()];

        if self.len() <= 1 {
            return steps;
        }

        let mut i = 1;

        while i < self.len() {
            if self[i] < self[i-1] {
                let tmp = self[i];
                self[i] = self[i-1];
                self[i-1] = tmp;

                steps.push(self.clone());

                if i > 1 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }

        return steps;
    }

    fn gnome_sort_stepped_and_timed(&mut self) -> (Vec<Vec<T>>, Duration){
        let time = Instant::now();

        let mut steps = vec![self.clone()];

        if self.len() <= 1 {
            return (steps, time.elapsed());
        }

        let mut i = 1;

        while i < self.len() {
            if self[i] < self[i-1] {
                let tmp = self[i];
                self[i] = self[i-1];
                self[i-1] = tmp;

                steps.push(self.clone());

                if i > 1 {
                    i -= 1;
                }
            } else {
                i += 1;
            }
        }

        return (steps, time.elapsed());
    }
}

/// The gnome sort algorithm.
///
/// Sorts a given `Vec` and returns the result.
pub fn gnome_sort<T>(mut arr: Vec<T>) -> Vec<T> 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    // If the array only contains one element, it's sorted by default.
    if arr.len() <= 1 {
        return arr;
    }

    // We start at one since the gnome can only 
    // compare the current pot with the previous one.
    let mut i = 1;

    while i < arr.len() {
        if arr[i] < arr[i-1] {
            let tmp = arr[i];
            arr[i] = arr[i-1];
            arr[i-1] = tmp;

            if i > 1 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    return arr;
}

/// The gnome sort algorithm but timed.
///
/// Sorts a given `Vec` and returns the result and the `Duration` of the process.
pub fn gnome_sort_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Duration) 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    // If the array only contains one element, it's sorted by default.
    if arr.len() <= 1 {
        return (arr, time.elapsed());
    }

    // We start at one since the gnome can only 
    // compare the current pot with the previous one.
    let mut i = 1;

    while i < arr.len() {
        if arr[i] < arr[i-1] {
            let tmp = arr[i];
            arr[i] = arr[i-1];
            arr[i-1] = tmp;

            if i > 1 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    (arr, time.elapsed())
}

/// The gnome sort algorithm but stepped.
///
/// Sorts a given `Vec` and returns the result and a `Vec` containing each step of the process.
pub fn gnome_sort_stepped<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>) 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let mut steps = vec![arr.clone()];

    // If the array only contains one element, it's sorted by default.
    if arr.len() <= 1 {
        return (arr, steps);
    }

    // We start at one since the gnome can only 
    // compare the current pot with the previous one.
    let mut i = 1;

    while i < arr.len() {
        if arr[i] < arr[i-1] {
            let tmp = arr[i];
            arr[i] = arr[i-1];
            arr[i-1] = tmp;

            // Push the current state to the list of steps.
            steps.push(arr.clone());

            if i > 1 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    (arr, steps)
}

/// The gnome sort algorithm but stepped _and_ timed.
///
/// Sorts a given `Vec` and returns the result and a `Vec` containing each step of the 
/// process, including the `Duration` of the process.
pub fn gnome_sort_stepped_and_timed<T>(mut arr: Vec<T>) -> (Vec<T>, Vec<Vec<T>>, Duration) 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let time = Instant::now();

    let mut steps = vec![arr.clone()];

    // If the array only contains one element, it's sorted by default.
    if arr.len() <= 1 {
        return (arr, steps, time.elapsed());
    }

    // We start at one since the gnome can only 
    // compare the current pot with the previous one.
    let mut i = 1;

    while i < arr.len() {
        if arr[i] < arr[i-1] {
            let tmp = arr[i];
            arr[i] = arr[i-1];
            arr[i-1] = tmp;

            // Push the current state to the list of steps.
            steps.push(arr.clone());

            if i > 1 {
                i -= 1;
            }
        } else {
            i += 1;
        }
    }

    (arr, steps, time.elapsed())
}
