pub mod person;
pub mod test;

// test
pub fn return_test() -> i64{
    return 999 as i64;
}

pub fn insert_into_arr(arr: &mut [i64], value: i64, index: usize) {
     arr[index] = value;
}

// secret func
fn secret_cant_be_accessed_in_other_mods(){
    println!("SECRET");
}


pub fn run(){
    // let mut temp1:i64 = 23;
    // let pointer_temp1: i64 = &temp1;
    let my_num: i32 = 10;
    let my_num_ptr: *const i32 = &my_num;
    let mut my_speed: i32 = 88;
    let my_speed_ptr: *mut i32 = &mut my_speed;
    let mut temp_arr1: [i64;4] = [1,2,3,4];
    println!("{:?}", temp_arr1);
    insert_into_arr(&mut temp_arr1, 999, 0);
    println!("{:?}", temp_arr1);
    // if true{
    //     let a = 90;
    // }
    // println!("{}", a); // error :: scope out of bounds
    let temp_tuple1: (char, i32, f64) = ('a', 23,43.65);
    let (temp_char1, temp_i32_1, temp_f64_1) = temp_tuple1;
    let temp_first_tuple1 = temp_tuple1.0;
    println!("{:?}", temp_tuple1);
    println!("{}", temp_char1);
    println!("{}", temp_first_tuple1);

    // array
    let temp_num_arr1 = [1,2,3,4,5];
    println!("{:?}", temp_num_arr1);
    let days = ["Monday","Tuesday","Wednesday","Thursday","Friday","Saturday","Sunday"];
    println!("{:?}",days);
    let mut temp_num_arr2: [i64;10]; 
    let mut temp_num_arr3: [i64;3]= [1,2,3]; 
    let temp_rand_arr1 = ["initial value"; 4];
    // so you can declare empty array
    println!("{:?}",temp_rand_arr1);
    // println!("{:?}",temp_num_arr2); // uninitialized (possibly)
    println!("{:?}",temp_num_arr3);

    // shadowing 
    let x = 90;
    let x = x + 90;
    let name = "what";
    let name = name.len(); 
    // can be done, so rust will use 
    // reusing the same name for the variable
    // x = 90; // error here cause we dont use let
    const TEMP_CONTANT:i64 = 9999;
    const TEMP_STRING: &str = "WAHT";
    println!("90 + 90 = {}", add_a_b_i64(90,90));
    let temp_statement = {
        let x = 9999; // statement : 
        // 9999 in let x
        // is an expression, cause it return 9999 
        // as a value
        x + 1 // expression
    };
    println!("temp_statement : {}", temp_statement);
    println!("{}",five_add_one() + 6);
    println!("five plus 10 = {}", five_plus_something(10));
}

fn add_a_b_i64(a:i64,b:i64)->i64{
    return a + b;
}
fn five_add_one()->u8{
    5 + 1 // expression
}
fn five_plus_something(some_number: i64)->i64{
    5 + some_number
}
// note : Function
// Statements are instructions that perform some 
// action and do not return a value. 

// Expressions evaluate to a resulting value. 
// Expressions do not include ending semicolons.
//  If you add a semicolon to the end of an 
// expression, you turn it into a statement, 
// and it will then not return a value.
