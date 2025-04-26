// Now that v2 implements PartialOrd it would be interesting to see how the code can be optimized
// Previously suggested that mutating the arrays instead of concatenating could improve the sorting, more specifically it would require less cloning

pub fn quick_sort_v3<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: PartialOrd + Clone + std::fmt::Debug,
{
    todo!()
}

// Based on quick_sort_v1 the following could be improved
// Generic types & comparator
// Mutate arrays instead of concatening

// The rust lang defines a trait called PartialOrd, see https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
// As long as the type implements this you can use operators (lt, le, gt, ge and ne) to compare
// Rust implements

pub fn quick_sort_v2<T: PartialOrd + Clone + std::fmt::Debug>(mut vec: Vec<T>) -> Vec<T> {
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

    if smaller.len() == 0 {
        return [vec![v], quick_sort_v2(larger)].concat();
    }

    return [quick_sort_v2(smaller), vec![v], quick_sort_v2(larger)].concat();
}

// Based on the 4 steps described in this article https://www.geeksforgeeks.org/quick-sort-algorithm/

// Quicksort uses divide and conquer to break the problem down into smaller problems
// It uses pivot points, choosen at ... from which all element smaller than are left and larger are right of the pivot point. The same is applied to the left over sub arrays recursively until the array is sorted.

// let unsorted_vec = vec![0, 1, 3, 2, 5, 6, 1, 6, 25, 6, 2];

pub fn quick_sort_v1(mut vec: Vec<i32>) -> Vec<i32> {
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
        return [vec![v], quick_sort_v1(larger)].concat();
    }

    return [quick_sort_v1(smaller), vec![v], quick_sort_v1(larger)].concat();
}
