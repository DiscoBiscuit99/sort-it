pub fn insertion_sort
    <T: std::cmp::PartialOrd
      + std::marker::Copy
      + std::clone::Clone>
(mut list: Vec<T>) -> Vec<T> {
    for i in 1..list.len() {
        let mut j = i-1;
        while list[j+1] < list[j] {
            list.swap(j, j+1);
            if j > 0 { 
                j -= 1; 
            }
        }
    } list
}

