use std::{
    io::{self, BufRead},
    time::Instant,
};

// use crate::generate_testfile::make_testfile;

// mod generate_testfile;
fn main() {
    let start = Instant::now();
    // make_testfile();
    let mut values = get_input();
    // let mut og = values.clone();
    // og.sort();
    let max_depth = (values.len() as f64).log2().floor() as usize * 3 as usize;
    eprintln!("Max: {}", max_depth);
    let len = values.len();
    smart_sort(&mut values, max_depth, 0, len - 1, len);
    // assert_eq!(values, og);
    // eprintln!("{:?}", values);
    // println!(
    //     "{}",
    //     values
    //         .iter()
    //         .map(|i| i.to_string())
    //         .collect::<Vec<String>>()
    //         .join(" ")
    // );
    let duration = start.elapsed();

    eprintln!("Time elapsed in expensive_function() is: {:?}", duration);
}

fn smart_sort(values: &mut [i32], max_depth: usize, low: usize, high: usize, length: usize) {
    if values.is_empty() {
        return;
    }

    if length <= 100 {
        insertion_sort(&mut values[low..=high])
    } else if max_depth == 0 {
        eprintln!("Heap");
        heap_sort(values)
    } else {
        if low < high {
            let pivot = partition(values, low, high);
            let length = high - low;
            smart_sort(values, max_depth - 1, low, pivot, length);
            smart_sort(values, max_depth - 1, pivot + 1, high, length);
        }
    }
}

//Hoare Partition
fn partition(values: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = values[low];
    let mut i: i32 = low as i32 - 1;
    let mut j: i32 = high as i32 + 1;
    loop {
        i += 1;
        while values[i as usize] < pivot {
            i += 1;
        }
        j -= 1;
        while values[j as usize] > pivot {
            j -= 1;
        }
        if i >= j {
            return j as usize;
        }
        values.swap(i as usize, j as usize);
    }
}

fn insertion_sort(values: &mut [i32]) {
    for j in 1..values.len() {
        let index = values[j];
        let mut i = (j as i8) - 1;

        while i >= 0 && values[i as usize] > index {
            values[(i + 1) as usize] = values[i as usize];
            i -= 1;
        }

        values[(i + 1) as usize] = index;
    }
}

fn left(i: usize) -> usize {
    2 * i
}
fn right(i: usize) -> usize {
    2 * i + 1
}
fn leaves(length: usize) -> usize {
    length / 2
}

fn heap_sort(mut values: &mut [i32]) {
    build_max_heap(&mut values);
    for i in (0..(values.len())).rev() {
        values.swap(0, i);
        max_heapify(&mut values[..i], 0);
    }
}

fn build_max_heap(values: &mut [i32]) {
    let leaves = leaves(values.len());
    for i in (0..(leaves)).rev() {
        max_heapify(values, i)
    }
}

fn max_heapify(values: &mut [i32], i: usize) {
    let l = left(i + 1) - 1;
    let r = right(i + 1) - 1;

    let mut largest = i;
    if l < values.len() {
        if values[i] < values[l] {
            largest = l;
        }
    }
    if r < values.len() {
        if values[i] < values[r] && values[r] > values[largest] {
            largest = r;
        }
    }

    if largest != i {
        values.swap(i, largest);
        max_heapify(values, largest);
    }
}

/***
 * Template to a Kattis solution.
 * See: https://open.kattis.com/help/rust
 * Author: Viola Söderlund <violaso@kth.se>
 */
fn get_input() -> Vec<i32> {
    let input = io::stdin();
    let values = input
        .lock()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    values
}

#[cfg(test)]
mod tests {
    use crate::smart_sort;

    #[test]
    fn easy_nr() {
        let mut res = vec![3, 6, 21, 3, 7, 1, 4, 7, 9, 2, 3, 7, 0];
        let length = res.len();
        smart_sort(&mut res, length, 0, length - 1);
        assert_eq!(res, [0, 1, 2, 3, 3, 3, 4, 6, 7, 7, 7, 9, 21])
    }
    #[test]
    fn basic() {
        let mut res = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        let length = res.len();
        smart_sort(&mut res, length, 0, length - 1);
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }

    #[test]
    fn empty() {
        let mut res = Vec::<i32>::new();
        let length = res.len();
        smart_sort(&mut res, length, 0, 0);
        assert_eq!(res, vec![]);
    }

    #[test]
    fn one_element() {
        let mut res = vec![1];
        let length = res.len();
        smart_sort(&mut res, length, 0, length - 1);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn pre_sorted() {
        let mut res = vec![1, 2, 3, 4];
        let length = res.len();
        smart_sort(&mut res, length, 0, length - 1);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }

    #[test]
    fn reverse_sorted() {
        let mut res = vec![4, 3, 2, 1];
        let length = res.len();
        smart_sort(&mut res, length, 0, length - 1);
        assert_eq!(res, vec![1, 2, 3, 4]);
    }
}
