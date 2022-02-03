# Mandatory readme

This crate provides various different sorting algorithms both implemented directly on 
any `Vec` implementing certain things and also as standalone functions.

# Examples

Using the trait implementations:

```rust
use sort_it::prelude::*;

fn main() {
    let mut v = vec![9, 8, 7, 6, 5, 4, 3, 2, 1];
    
    println!("original v: {:?}", v.clone()); 
    
    v.gnome_sort(); // sorts `v` via gnome sort (with the trait implementation).
    
    println!("sorted v: {:?}", v);
}
```

Without using the trait implementations:

```rust
use sort_it::prelude::*;
use rand::prelude::*;

fn main() {
    let mut rng = rand::thread_rng();
    
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    v.shuffle(&mut rng); // randomly shuffles `v`.
    
    let s = merge_sort(v.clone()); // returns a sorted copy of `v` via merge sort (without the trait implementation).
    
    println!("v: {:?}, s: {:?}", v, s);
}
```

# Implemented sorting algorithms:

* Bogosort
* Bubble Sort
* Gnome Sort
* Insertion Sort
* Merge Sort
* Selection Sort
* Slowsort
* Stooge Sort

Have fun sorting things in different ways.
