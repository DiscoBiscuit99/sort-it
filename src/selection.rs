/// Takes a vec of any type implementing `std::cmp::PartialOrd` and returns the ordered list in ascending order.
pub fn selection_sort
    <T: std::cmp::PartialOrd
      + std::marker::Copy
      + std::clone::Clone>
(mut list: Vec<T>) -> Vec<T> {
    // Loop through the list twice and check the elements against 
    // each other; if an element is greater than another, swap them.
    for i in 0..list.len() {
        for j in i..list.len() {
            if list[j] < list[i] {
                list.swap(j, i);
            }
        }
    }

    list
}

