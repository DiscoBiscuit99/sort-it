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

/// Auxiliary function.
fn stooge_sort_rec<T>(arr: &mut Vec<T>, i: usize, j: usize)
    where T: PartialEq + PartialOrd + Clone + Copy,
{
    let length = arr.len();

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

