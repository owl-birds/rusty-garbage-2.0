use rand::Rng;
use std::mem;
use std::time::Instant;

fn add_items(arrays: &mut Vec<i64>, items: i64) {
    arrays.push(items);
}
fn linear_search(arrays: &Vec<i64>, num: i64) -> i64 {
    let mut idx = -1;
    for i in 0..arrays.len() {
        if arrays[i] == num {
            idx = i as i64;
        };
    }
    return idx;
}
fn generate_random_vector_i64(length: usize, max_num: i64) -> Vec<i64> {
    let mut result_arr: Vec<i64> = vec![];
    // rand::thread_rng().gen_range(1..101)
    let mut i: usize = 0;
    while i < length {
        let temp_num = rand::thread_rng().gen_range(0..max_num);
        result_arr.push(temp_num);
        i += 1;
    }
    return result_arr;
}

pub fn run() {
    let start = Instant::now();
    println!("big_o.rs");
    let end = Instant::now();
    println!("{:?}", start.elapsed());
    println!("{:?}", start.saturating_duration_since(end)); // 0ns
    println!("{:?}", end.saturating_duration_since(start));

    // BIG O
    let mut arrays: Vec<i64> = vec![];
    let arrays_size = 10;
    add_items(&mut arrays, 999);
    println!("this vector occupy : {} bytes", mem::size_of_val(&arrays));
    println!("{:?}", arrays);
    add_items(&mut arrays, 1999);
    println!("this vector occupy : {} bytes", mem::size_of_val(&arrays));
    println!("{:?}", arrays);
    add_items(&mut arrays, 29999);
    println!("this vector occupy : {} bytes", mem::size_of_val(&arrays));
    println!("{:?}", arrays);

    println!("BIG O");
    // O(1)
    // println!("{}", arrays[0]);
    let start = Instant::now();
    add_items(&mut arrays, 2999999);
    println!("{:?}", start.elapsed());
    println!("{:?}", arrays);
    // O(n)
    let start = Instant::now();
    let index = linear_search(&arrays, 29999);
    println!("{:?}", start.elapsed());
    println!("index of 29999 in {:?} is : {}", arrays, index);
    // generating random array of num
    let arr1 = generate_random_vector_i64(100000, 1000);
    let arr2 = generate_random_vector_i64(200000, 1000);
    let arr3 = generate_random_vector_i64(300000, 1000);
    let start = Instant::now();
    let index = linear_search(&arr1, 20);
    println!("arr1 : {:?} : idx {}", start.elapsed(), index);
    let start = Instant::now();
    let index = linear_search(&arr2, 20);
    println!("arr2 : {:?} : idx {}", start.elapsed(), index);
    let start = Instant::now();
    let index = linear_search(&arr3, 20);
    println!("arr1 : {:?} : idx {}", start.elapsed(), index);
    // so the number of elements is having direct relations
    // O(n)
}
