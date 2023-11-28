//  cargo run --bin chapter3-array-vec
use rand::Rng;
fn main() {
    let arr = [9; 10];
    println!("array {:?}", arr);
    let arr = vec![19, 5, 7, 98, 76, 4, 87, 5];
    println!("random access vec {:?} => {:?}", arr, random_access(&arr));
    let mut arr = vec![8; 10];
    let clone = arr.clone();
    insert(&mut arr, 3, 5);
    println!("insert {:?} {} @ {} => {:?}", clone, 5, 3, arr);
    remove(&mut arr, 3);
    println!("remove {:?} @ {} => {:?}", clone, 3, arr);
    traverse(&arr);
    println!(
        "in {:?} find {} => {:?}; find {} => {:?}",
        arr,
        3,
        find(&arr, 3),
        8,
        find(&arr, 8),
    );
    println!("enlarge {:?} + {} => {:?}", clone, 5, extend(&clone, 5));
}

fn random_access(arr: &[i32]) -> (usize, i32) {
    let index = rand::thread_rng().gen_range(0..arr.len());
    (index, arr[index])
}

fn insert(arr: &mut Vec<i32>, index: usize, value: i32) {
    if index >= arr.len() {
        panic!("index out of bounds");
    }
    for i in (index + 1..arr.len()).rev() {
        arr[i] = arr[i - 1]
    }
    arr[index] = value;
}

fn remove(arr: &mut Vec<i32>, index: usize) {
    if index >= arr.len() {
        panic!("index out of bounds");
    }
    for i in index..arr.len() - 1 {
        arr[i] = arr[i + 1]
    }
    arr.pop();
}

fn traverse(arr: &[i32]) {
    for i in 0..arr.len() {
        print!("{}", arr[i]);
        if i != arr.len() - 1 {
            print!(", ")
        }
    }
    println!("");
}

fn find(arr: &[i32], target: i32) -> Option<usize> {
    for i in 0..arr.len() {
        if arr[i] == target {
            return Some(i);
        }
    }
    None
}

fn extend(arr: &Vec<i32>, enlarge: usize) -> Vec<i32> {
    let mut new_arr = vec![0; arr.len() + enlarge];
    for i in 0..arr.len() {
        new_arr[i] = arr[i];
    }
    new_arr
}
