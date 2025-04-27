use quick_sort::{quick_sort_v1, quick_sort_v2, quick_sort_v3, quick_sort_v4, quick_sort_v5};

mod quick_sort;

fn main() {
    let unsorted_vec = vec![0, 1, 3, 2, 5, 6, 1, 6, 25, 6, 2];
    let sorted = quick_sort_v1(unsorted_vec);
    println!("{:?}", sorted);

    let unsorted_vec = vec![0, 1, 3, 2, 5, 6, 1, 6, 25, 6, 2];
    let sorted = quick_sort_v2(unsorted_vec);
    println!("{:?}", sorted);

    let unsorted_vec = vec!['a', 'd', 'a', 'b', 'c', 'a', 'f'];
    let sorted = quick_sort_v2(unsorted_vec);
    println!("{:?}", sorted);

    let unsorted_vec = vec![0, 1, 3, 2, 5, 6, 1, 6, 25, 6, 2];
    let sorted = quick_sort_v3(unsorted_vec);
    println!("{:?}", sorted);

    let unsorted_vec = vec!['a', 'd', 'a', 'b', 'c', 'a', 'f'];
    let sorted = quick_sort_v3(unsorted_vec);
    println!("{:?}", sorted);

    let unsorted_vec = vec![0, 1, 3, 2, 5, 6, 1, 6, 25, 6, 2];
    let sorted = quick_sort_v4(unsorted_vec);
    println!("{:?}", sorted);

    let unsorted_vec = vec!['a', 'd', 'a', 'b', 'c', 'a', 'f'];
    let sorted = quick_sort_v4(unsorted_vec);
    println!("{:?}", sorted);

    let mut unsorted_vec = vec![0, 1, 3, 2, 5, 6, 1, 6, 25, 6, 2];
    let sorted = quick_sort_v5(&mut unsorted_vec);
    println!("{:?}", sorted);

    let mut unsorted_vec = vec!['a', 'd', 'a', 'b', 'c', 'a', 'f'];
    let sorted = quick_sort_v5(&mut unsorted_vec);
    println!("{:?}", sorted);
}
