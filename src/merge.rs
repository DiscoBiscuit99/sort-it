pub fn merge_sort<T: std::cmp::PartialOrd
                   + std::marker::Copy
                   + std::clone::Clone
                   + std::fmt::Debug>
(list: Vec<T>) -> Vec<T> {
    // Sort the list if necessary.
    if list.len() > 1 {
        let mut left = vec![];
        let mut right = vec![];

        for (i, x) in list.iter().enumerate() {
            if i < list.len() / 2 {
                left.push(*x);
            } else {
                right.push(*x);
            }
        }

        left = merge_sort(left);
        right = merge_sort(right);

        return merge(left, right)
    }

    // If the list contains one or less elements, the list is considered sorted.
    list
}

fn merge<T: std::cmp::PartialOrd
          + std::marker::Copy
          + std::clone::Clone
          + std::fmt::Debug>
(mut left: Vec<T>, mut right: Vec<T>) -> Vec<T> { 
    let mut sorted_list = vec![];

    while left.len() > 0 && right.len() > 0 {
        if left[0] <= right[0] {
            sorted_list.push(left[0]);
            left.remove(0);
        } else {
            sorted_list.push(right[0]);
            right.remove(0);
        }
    }

    while left.len() > 0 {
        sorted_list.push(left[0]);
        left.remove(0);
    }
    while right.len() > 0 {
        sorted_list.push(right[0]);
        right.remove(0);
    }

    sorted_list
}

