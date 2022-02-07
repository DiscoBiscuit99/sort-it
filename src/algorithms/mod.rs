pub mod bogosort;
pub mod bubble_sort;
pub mod gnome_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quicksort;
pub mod selection_sort;
pub mod slowsort;
pub mod stooge_sort;
pub mod tree_sort;

pub use bogosort::*;
pub use bubble_sort::*;
pub use gnome_sort::*;
pub use insertion_sort::*;
pub use merge_sort::*;
pub use quicksort::*;
pub use selection_sort::*;
pub use slowsort::*;
pub use stooge_sort::*;
pub use tree_sort::*;

/// A trait providing the `is_sorted` method on `Vec`'s implementing `T`.
pub trait IsSorted<T: PartialEq + PartialOrd + Clone + Copy> {
    /// Returns whether the given `Vec` is sorted or not.
    fn is_sorted(&self) -> bool;
}

/// The trait implementation providing the `is_sorted` method.
impl<T> IsSorted<T> for Vec<T>
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    fn is_sorted(&self) -> bool {
        let mut is_sorted = true;

        for i in 1..self.len() {
            if self[i] < self[i-1] {
                is_sorted = false;
            }
        }

        return is_sorted;
    }
}

