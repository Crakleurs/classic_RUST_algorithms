use rand::Rng;
use std::time::Instant;

fn main() {
    let mut arr = [0; 100];
    generate_random_array(&mut arr);

    shuffle(&mut arr);
    let mut now = Instant::now();
    bubble_sort(&mut arr);
    let mut new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
    let reference_array = arr.clone();
    println!("{}", compare_two_arrays(&reference_array, &arr));

    shuffle(&mut arr);
    now = Instant::now();
    insertion_sort(&mut arr);
    new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
    println!("{}", compare_two_arrays(&reference_array, &arr));

    shuffle(&mut arr);
    now = Instant::now();
    merge_sort(&mut arr);
    new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
    println!("{}", compare_two_arrays(&reference_array, &arr));

    shuffle(&mut arr);
    let array_length = arr.len() - 1 ;
    now = Instant::now();
    quick_sort(&mut arr, 0, array_length);
    new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
    println!("{}", compare_two_arrays(&reference_array, &arr));

    shuffle(&mut arr);
    now = Instant::now();
    heap_sort(&mut arr);
    new_now = Instant::now();
    println!("{:?}", new_now.duration_since(now));
    println!("{}", compare_two_arrays(&reference_array, &arr));

}

fn fibonacci(n: u64) -> u64 {
    let mut u0 : u64 = 0;
    let mut u1 : u64 = 1;
    let mut i : u64 = 2;
    let mut result : u64 = 1;
    if n <= 1 {
        return n;
    }
    while i <= n {
        result = u0 + u1;
        u0 = u1;
        u1 = result;
        i += 1;
    }
    result
}

fn fibonacci_recursive(n: u64) -> u64 {
    if n <= 1 {
        return n;
    }
    return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2);
}

fn bubble_sort(array_to_sort: &mut [i32]){
    for j in (1..(array_to_sort.len() + 1)).rev() {
        for i in 1..j  {
            if array_to_sort[i - 1] > array_to_sort[i] {
                array_to_sort.swap(i-1, i);
            }
        }
    }
}

fn insertion_sort(array_to_sort: &mut [i32]){
    for i in 1..array_to_sort.len() {
        let mut j = i;
        while j > 0 && array_to_sort[j-1] > array_to_sort[j] {
            array_to_sort.swap(j-1, j);
            j -= 1;
        }
    }
}

fn merge_sort<T: Copy + Ord>(array_to_sort: &mut [T]){
    let array_len = array_to_sort.len();
    if array_len <= 1 {
        return ;
    }
    let middle_index: usize = array_len/2;
    merge_sort(&mut array_to_sort[..middle_index]);
    merge_sort(&mut array_to_sort[middle_index..]);

    let mut vector: Vec<T> = array_to_sort.to_vec();

    merge(&array_to_sort[..middle_index], &array_to_sort[middle_index..], &mut vector);

    array_to_sort.copy_from_slice(&vector);
}

fn merge<T: Copy + PartialOrd>(first_array: &[T], second_array: &[T], vector: &mut Vec<T>){
    assert_eq!(first_array.len() + second_array.len(), vector.len());
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < first_array.len() && j < second_array.len() {
        if first_array[i] < second_array[j] {
            vector[k] = first_array[i];
            k += 1;
            i += 1;
        }else {
            vector[k] = second_array[j];
            k += 1;
            j += 1;
        }
    }
    if i < first_array.len() {
        vector[k..].copy_from_slice(&first_array[i..]);
    }
    if j < second_array.len() {
        vector[k..].copy_from_slice(&second_array[j..]);
    }
}

fn quick_sort(array_to_sort: &mut [i32], first_index: usize, last_index: usize){
    if first_index < last_index {
        let mut rng = rand::thread_rng();
        let mut pivot= rng.gen_range(first_index..(last_index + 1));
        pivot = partition(array_to_sort, first_index, last_index, pivot);

        if pivot > first_index {
            quick_sort(array_to_sort, first_index, pivot - 1);
        }
        if pivot < last_index {
            quick_sort(array_to_sort, pivot + 1, last_index);
        }
    }
}

fn partition(array_to_sort: &mut [i32], first_index: usize, last_index: usize, pivot_index: usize) -> usize {
    array_to_sort.swap(pivot_index, last_index);

    let mut j = first_index;
    for i in first_index..last_index {
        if array_to_sort[i] <= array_to_sort[last_index] {
            array_to_sort.swap(i, j);
            j += 1;
        }
    }
    array_to_sort.swap(last_index, j);
    return j;
}

//  Fisher–Yates shuffle
fn shuffle(array_to_shuffle: &mut [i32]){
    let mut j;
    let mut rng = rand::thread_rng();
    for i in (1..(array_to_shuffle.len())).rev() {
        j = rng.gen_range(0..i);
        array_to_shuffle.swap(i, j);
    }
}

fn generate_random_array(array_to_generate: &mut [i32]){
    let mut rng = rand::thread_rng();
    for i in 0..array_to_generate.len(){
        array_to_generate[i] = rng.gen::<u8>() as i32;
    }
}

fn compare_two_arrays(first_array: &[i32], second_array: &[i32]) -> bool{
    assert_eq!(first_array.len(), second_array.len());
    for i in 0..first_array.len(){
        if first_array[i] != second_array[i] {
            return false;
        }
    }
    return true;
}

fn heap_sort(array: &mut[i32]){
    heapify(array);
    let mut k = array.len() - 1;

    loop {
        if k > 0 {
            array.swap(0, k);
            sift(array, 0, k - 1);
            k -= 1;
        }else {
            break;
        }
    }
}

fn sift(array: &mut[i32], node: usize, len: usize) {
    let mut father = node;
    let mut child = 2 * father + 1;

    if (child < len) && (array[child] < array[child +1]) {
        child += 1;
    }
    if (child < len) && (array[father] < array[child]) {
        array.swap(father, child);
        sift(array, child, len)
    }
}

fn heapify(array: &mut[i32]) {
    let mut i = array.len()/2;
    loop {
        sift(array, i, array.len() - 1);
        if i > 0{
            i -= 1;
        }else {
            break
        }
    }
}