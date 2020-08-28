pub fn bubble_sort
    <T: std::cmp::PartialOrd>
(mut list: Vec<T>) -> Vec<T> {
    // Loop through the list and compare the elements in 
    // pairs until an iteration causes no swaps.
    loop {
        let mut sorted = true;
        for i in 0..list.len() {
            if i < list.len()-1 {
                if list[i] > list[i+1] {
                    list.swap(i, i+1);
                    sorted = false;
                }
            }
        }

        if sorted { break; }
    }

    list
}

