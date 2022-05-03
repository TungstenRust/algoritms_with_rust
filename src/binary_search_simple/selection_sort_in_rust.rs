#![allow (warnings)]
//Linked lists are odd for Rust lang because Selection Sort and Linked Lists are messing with memory too much
use std::fmt::Debug;

pub fn selection_sort<T>(arr: &mut [T])
    where T: Ord + Debug {
    println!("Original: {:?}", arr);
    for i in 0..arr.len()-1 {
        let mut min_index =i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        arr.swap(i, min_index);
        println!("{:?}", arr);
        println!("---------------------");
    }
}

fn main() {
    let mut nums = [50, 4, 30, 5, 20];
    selection_sort( & mut nums);
    println!("Sorted: {:?}", nums);
}