// Based on the 4 steps described in this article https://www.geeksforgeeks.org/quick-sort-algorithm/

// Quicksort uses divide and conquer to break the problem down into smaller problems
// It uses pivot points, choosen at ... from which all element smaller than are left and larger are right of the pivot point. The same is applied to the left over sub arrays recursively until the array is sorted.

fn main() {
    let unsorted_vec = vec![0, 1, 3, 2, 5, 6, 1, 6, 25, 6, 2];
    let sorted = quick_sort(unsorted_vec);
    println!("{:?}", sorted);
}

fn quick_sort(mut vec: Vec<i32>) -> Vec<i32> {
    if vec.len() <= 1 {
        return vec;
    }
    let v = vec.remove(0);
    let mut smaller = vec![];
    let mut larger = vec![];
    for n in vec {
        if n < v {
            smaller.push(n)
        } else {
            larger.push(n)
        }
    }
    println!("{:?} {:?}", smaller, larger);

    if smaller.len() == 0 {
        return [vec![v], quick_sort(larger)].concat();
    }

    return [quick_sort(smaller), vec![v], quick_sort(larger)].concat();
}
