/// The slowsort algorithm.
///
/// Sorts the given `Vec` and returns the result.
pub fn slowsort<T>(mut arr: Vec<T>) -> Vec<T>
    where T: PartialEq + PartialOrd + Clone + Copy + std::fmt::Debug,
{
    // NOTE: this may be too serious of an optimization 
    //       for the kind of algorithm this is.
    if arr.len() <= 1 {
        return arr;
    }

    let length = arr.len();
    slowsort_rec(&mut arr, 0, length - 1);

    return arr;
}

/// Auxiliary function.
fn slowsort_rec<T>(arr: &mut Vec<T>, i: usize, j: usize) 
    where T: PartialEq + PartialOrd + Clone + Copy + std::fmt::Debug,
{
    if i >= j {
        return;
    }

    let m = (i + j) / 2;

    slowsort_rec(arr, i, m);
    slowsort_rec(arr, m+1, j);

    if arr[m] > arr[j] {
        swap(arr, m, j);
    }

    slowsort_rec(arr, i, j-1);
}

/// Utility function.
///
/// Swaps the elements of the given `Vec` at the given indices.
fn swap<T>(arr: &mut Vec<T>, i: usize, j: usize) 
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let tmp = arr[i];
    arr[i] = arr[j];
    arr[j] = tmp;
}

