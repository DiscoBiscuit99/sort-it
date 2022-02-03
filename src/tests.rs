use crate::algorithms::{
    self,
    Bogosort, 
    BubbleSort,
    GnomeSort,
    InsertionSort,
    MergeSort,
    SelectionSort,
    Slowsort,
    StoogeSort,
};

#[test]
fn bogosort() {
    let mut arr_0 = vec![2, 1, 3];
    let sorted_0  = vec![1, 2, 3];

    let mut arr_1 = vec![1];
    let sorted_1  = vec![1];

    let mut arr_2 = vec![1.2, 0.1];
    let sorted_2  = vec![0.1, 1.2];

    assert_eq!(algorithms::bogosort(arr_0.clone()), sorted_0);
    assert_eq!(algorithms::bogosort(arr_1.clone()), sorted_1);
    assert_eq!(algorithms::bogosort(arr_2.clone()), sorted_2);

    arr_0.bogosort();
    arr_1.bogosort();
    arr_2.bogosort();

    assert_eq!(arr_0, sorted_0);
    assert_eq!(arr_1, sorted_1);
    assert_eq!(arr_2, sorted_2);
}

#[test]
fn bubble_sort() {
    let mut arr_0 = vec![2, 1, 4, 3];
    let sorted_0  = vec![1, 2, 3, 4];

    let mut arr_1 = vec![1];
    let sorted_1  = vec![1];

    let mut arr_2 = vec![0.1, 3.1, 2.3, 1.2];
    let sorted_2  = vec![0.1, 1.2, 2.3, 3.1];

    assert_eq!(algorithms::bubble_sort(arr_0.clone()), sorted_0);
    assert_eq!(algorithms::bubble_sort(arr_1.clone()), sorted_1);
    assert_eq!(algorithms::bubble_sort(arr_2.clone()), sorted_2);
    
    arr_0.bubble_sort();
    arr_1.bubble_sort();
    arr_2.bubble_sort();

    assert_eq!(arr_0, sorted_0);
    assert_eq!(arr_1, sorted_1);
    assert_eq!(arr_2, sorted_2);
}

#[test]
fn gnome_sort() {
    let mut arr_0 = vec![2, 1, 4, 3];
    let sorted_0  = vec![1, 2, 3, 4];

    let mut arr_1 = vec![1];
    let sorted_1  = vec![1];

    let mut arr_2 = vec![0.1, 3.1, 2.3, 1.2];
    let sorted_2  = vec![0.1, 1.2, 2.3, 3.1];

    assert_eq!(algorithms::gnome_sort(arr_0.clone()), sorted_0);
    assert_eq!(algorithms::gnome_sort(arr_1.clone()), sorted_1);
    assert_eq!(algorithms::gnome_sort(arr_2.clone()), sorted_2);

    arr_0.gnome_sort();
    arr_1.gnome_sort();
    arr_2.gnome_sort();

    assert_eq!(arr_0, sorted_0);
    assert_eq!(arr_1, sorted_1);
    assert_eq!(arr_2, sorted_2);
}

#[test]
fn insertion_sort() {
    let mut arr_0 = vec![2, 1, 4, 3];
    let sorted_0  = vec![1, 2, 3, 4];

    let mut arr_1 = vec![1];
    let sorted_1  = vec![1];

    let mut arr_2 = vec![0.1, 3.1, 2.3, 1.2];
    let sorted_2  = vec![0.1, 1.2, 2.3, 3.1];

    assert_eq!(algorithms::insertion_sort(arr_0.clone()), sorted_0);
    assert_eq!(algorithms::insertion_sort(arr_1.clone()), sorted_1);
    assert_eq!(algorithms::insertion_sort(arr_2.clone()), sorted_2);

    arr_0.insertion_sort();
    arr_1.insertion_sort();
    arr_2.insertion_sort();

    assert_eq!(arr_0, sorted_0);
    assert_eq!(arr_1, sorted_1);
    assert_eq!(arr_2, sorted_2);
}

#[test]
fn merge_sort() {
    let mut arr_0 = vec![2, 1, 4, 3];
    let sorted_0  = vec![1, 2, 3, 4];

    let mut arr_1 = vec![1];
    let sorted_1  = vec![1];

    let mut arr_2 = vec![0.1, 3.1, 2.3, 1.2];
    let sorted_2  = vec![0.1, 1.2, 2.3, 3.1];

    assert_eq!(algorithms::merge_sort(arr_0.clone()), sorted_0);
    assert_eq!(algorithms::merge_sort(arr_1.clone()), sorted_1);
    assert_eq!(algorithms::merge_sort(arr_2.clone()), sorted_2);

    arr_0.merge_sort();
    arr_1.merge_sort();
    arr_2.merge_sort();

    assert_eq!(arr_0, sorted_0);
    assert_eq!(arr_1, sorted_1);
    assert_eq!(arr_2, sorted_2);
}

#[test]
fn quicksort() {
    todo!()
}

#[test]
fn selection_sort() {
    let mut arr_0 = vec![2, 1, 4, 3];
    let sorted_0  = vec![1, 2, 3, 4];

    let mut arr_1 = vec![1];
    let sorted_1  = vec![1];

    let mut arr_2 = vec![0.1, 3.1, 2.3, 1.2];
    let sorted_2  = vec![0.1, 1.2, 2.3, 3.1];

    assert_eq!(algorithms::selection_sort(arr_0.clone()), sorted_0);
    assert_eq!(algorithms::selection_sort(arr_1.clone()), sorted_1);
    assert_eq!(algorithms::selection_sort(arr_2.clone()), sorted_2);

    arr_0.selection_sort();
    arr_1.selection_sort();
    arr_2.selection_sort();

    assert_eq!(arr_0, sorted_0);
    assert_eq!(arr_1, sorted_1);
    assert_eq!(arr_2, sorted_2);
}

#[test]
fn slowsort() {
    let mut arr_0 = vec![2, 1, 4, 3];
    let sorted_0  = vec![1, 2, 3, 4];

    let mut arr_1 = vec![1];
    let sorted_1  = vec![1];

    let mut arr_2 = vec![0.1, 3.1, 2.3, 1.2];
    let sorted_2  = vec![0.1, 1.2, 2.3, 3.1];

    assert_eq!(algorithms::slowsort(arr_0.clone()), sorted_0);
    assert_eq!(algorithms::slowsort(arr_1.clone()), sorted_1);
    assert_eq!(algorithms::slowsort(arr_2.clone()), sorted_2);

    arr_0.slowsort();
    arr_1.slowsort();
    arr_2.slowsort();

    assert_eq!(arr_0, sorted_0);
    assert_eq!(arr_1, sorted_1);
    assert_eq!(arr_2, sorted_2);
}

#[test]
fn stooge_sort() {
    let mut arr_0 = vec![2, 1, 4, 3];
    let sorted_0  = vec![1, 2, 3, 4];

    let mut arr_1 = vec![1];
    let sorted_1  = vec![1];

    let mut arr_2 = vec![0.1, 3.1, 2.3, 1.2];
    let sorted_2  = vec![0.1, 1.2, 2.3, 3.1];

    assert_eq!(algorithms::stooge_sort(arr_0.clone()), sorted_0);
    assert_eq!(algorithms::stooge_sort(arr_1.clone()), sorted_1);
    assert_eq!(algorithms::stooge_sort(arr_2.clone()), sorted_2);

    arr_0.stooge_sort();
    arr_1.stooge_sort();
    arr_2.stooge_sort();

    assert_eq!(arr_0, sorted_0);
    assert_eq!(arr_1, sorted_1);
    assert_eq!(arr_2, sorted_2);
}

#[test]
fn tree_sort() {
    unimplemented!()
}
