// Now it would be interesting to test different pivots

pub fn quick_sort_v4<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: PartialOrd + Clone + Copy + std::fmt::Debug,
{
    fn partition<T>(vec: &mut Vec<T>, first: isize, last: isize) -> isize
    where
        T: PartialOrd + Clone + Copy + std::fmt::Debug,
    {
        let mut li = first - 1;
        let mut ri = last + 1;

        // PIVOT CANNOT BE LAST
        let pivot = vec[rand::random_range(first as usize..last as usize)];

        loop {
            li += 1;
            while vec[li as usize] < pivot {
                li += 1
            }
            ri -= 1;
            while vec[ri as usize] > pivot {
                ri -= 1
            }
            if li >= ri {
                return ri;
            }
            vec.swap(li as usize, ri as usize);
        }
    }

    fn sort<T>(mut vec: &mut Vec<T>, first: isize, last: isize) -> &mut Vec<T>
    where
        T: PartialOrd + Clone + Copy + std::fmt::Debug,
    {
        if first < last {
            let the_divide = partition(&mut vec, first, last);
            sort(&mut vec, first, the_divide);
            sort(&mut vec, the_divide + 1, last as isize);
        }
        return vec;
    }

    let last = vec.len() - 1;
    return sort(&mut vec, 0, last as isize).to_vec();
}

// Now that v2 implements PartialOrd it would be interesting to see how the code can be optimized
// Previously suggested that mutating the arrays instead of concatenating could improve the sorting, more specifically it would require less cloning

// Using the criterion crate I can benchmark the and compare the v2 against c3. Criterions black_box function avoid optimizations that could give unfair comparisons. It would be important to determine different
// Using criterion table I can easily document the difference
// The benchmarks should involve testing: correctness before benching then, different input sizes (vec length & item types), different types, and a control e.g. std sort

pub fn quick_sort_v3<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: PartialOrd + Clone + Copy + std::fmt::Debug,
{
    fn partition<T>(vec: &mut Vec<T>, first: isize, last: isize) -> isize
    where
        T: PartialOrd + Clone + Copy + std::fmt::Debug,
    {
        let mut li = first - 1;
        let mut ri = last + 1;

        // PIVOT CANNOT BE LAST
        let pivot = vec[first as usize];

        loop {
            li += 1;
            while vec[li as usize] < pivot {
                li += 1
            }
            ri -= 1;
            while vec[ri as usize] > pivot {
                ri -= 1
            }
            if li >= ri {
                return ri;
            }
            vec.swap(li as usize, ri as usize);
        }
    }

    fn sort<T>(mut vec: &mut Vec<T>, first: isize, last: isize) -> &mut Vec<T>
    where
        T: PartialOrd + Clone + Copy + std::fmt::Debug,
    {
        if first < last {
            let the_divide = partition(&mut vec, first, last);
            sort(&mut vec, first, the_divide);
            sort(&mut vec, the_divide + 1, last as isize);
        }
        return vec;
    }

    let last = vec.len() - 1;
    return sort(&mut vec, 0, last as isize).to_vec();
}

// Based on quick_sort_v1 the following could be improved
// Generic types & comparator
// Mutate arrays instead of concatening

// The rust lang defines a trait called PartialOrd, see https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
// As long as the type implements this you can use operators (lt, le, gt, ge and ne) to compare
// See rust PartialOrd implementors https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#implementors

pub fn quick_sort_v2<T: PartialOrd + Clone + std::fmt::Debug>(mut vec: Vec<T>) -> Vec<T> {
    if vec.len() <= 1 {
        return vec;
    }
    let pivot = vec.remove(0);
    let mut smaller = vec![];
    let mut larger = vec![];
    for n in vec {
        if n < pivot {
            smaller.push(n)
        } else {
            larger.push(n)
        }
    }

    if smaller.len() == 0 {
        return [vec![pivot], quick_sort_v2(larger)].concat();
    }

    return [quick_sort_v2(smaller), vec![pivot], quick_sort_v2(larger)].concat();
}

// Based on the 4 steps described in this article https://www.geeksforgeeks.org/quick-sort-algorithm/

// Quicksort uses divide and conquer to break the problem down into smaller problems
// It uses pivot points, choosen at ... from which all element smaller than are left and larger are right of the pivot point. The same is applied to the left over sub arrays recursively until the array is sorted.

// let unsorted_vec = vec![0, 1, 3, 2, 5, 6, 1, 6, 25, 6, 2];

pub fn quick_sort_v1(mut vec: Vec<i32>) -> Vec<i32> {
    if vec.len() <= 1 {
        return vec;
    }
    let pivot = vec.remove(0);
    let mut smaller = vec![];
    let mut larger = vec![];
    for n in vec {
        if n < pivot {
            smaller.push(n)
        } else {
            larger.push(n)
        }
    }

    if smaller.len() == 0 {
        return [vec![pivot], quick_sort_v1(larger)].concat();
    }

    return [quick_sort_v1(smaller), vec![pivot], quick_sort_v1(larger)].concat();
}
