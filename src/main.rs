use sort_it::prelude::*;

fn main() {
    let arr = vec![4, 2, 1, 5, 6];

    let (merge_sorted, merge_duration) = merge_sort_timed(arr.clone());
    let (gnome_sorted, gnome_duration) = gnome_sort_timed(arr.clone());
    let (insertion_sorted, insertion_duration) = insertion_sort_timed(arr.clone());
    let (selection_sorted, selection_duration) = insertion_sort_timed(arr.clone());
    let (bogo_sorted, bogo_duration) = bogosort_timed(arr.clone());

    println!("unsorted: {:?}", arr);
    println!("sorted by merge: {:?}  (~{}ns)", merge_sorted, merge_duration.as_nanos());
    println!("sorted by gnome: {:?}  (~{}ns)", gnome_sorted, gnome_duration.as_nanos());
    println!("sorted by insertion: {:?}  (~{}ns)", insertion_sorted, insertion_duration.as_nanos());
    println!("sorted by selection: {:?}  (~{}ns)", selection_sorted, selection_duration.as_nanos());
    println!("sorted by bogo: {:?}  (~{}ns)", bogo_sorted, bogo_duration.as_nanos());

    println!();

    let (merge_sorted, merge_steps, merge_duration) = merge_sort_stepped_and_timed(arr.clone());
    let (gnome_sorted, gnome_steps, gnome_duration) = gnome_sort_stepped_and_timed(arr.clone());
    let (insertion_sorted, insertion_steps, insertion_duration) = insertion_sort_stepped_and_timed(arr.clone());
    let (selection_sorted, selection_steps, selection_duration) = selection_sort_stepped_and_timed(arr.clone());
    let (bogo_sorted, bogo_steps, bogo_duration) = bogosort_stepped_and_timed(arr.clone());

    println!("sorted by merge: {:?}, steps: {:?}  (~{}ns)", merge_sorted, merge_steps, merge_duration.as_nanos());
    println!("sorted by gnome: {:?}, steps: {:?}  (~{}ns)", gnome_sorted, gnome_steps, gnome_duration.as_nanos());
    println!("sorted by insertion: {:?}, steps: {:?} (~{}ns)", insertion_sorted, insertion_steps, insertion_duration.as_nanos());
    println!("sorted by selection: {:?}, steps: {:?} (~{}ns)", selection_sorted, selection_steps, selection_duration.as_nanos());
    println!("sorted by bogo: {:?}, steps: {:?} (~{}ns)", bogo_sorted, bogo_steps, bogo_duration.as_nanos());
}

