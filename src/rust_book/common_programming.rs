pub fn fahrenheit_to_celcius(temp: f64) -> f64 {
    return (temp - 32.0) * 5.0 / 9.0;
}
pub fn fib_number_at(at: i64) -> i64 {
    if at == 0 {
        return 0;
    }
    let mut prev_num = 0;
    let mut curr_num = 1;
    let mut curr_at = 1;
    while curr_at < at {
        let temp_num = curr_num;
        curr_num = curr_num + prev_num;
        prev_num = temp_num;
        curr_at += 1;
    }
    return curr_num;
}
pub fn common_progamming_concepts() {
    //variables are immutable by default
    println!("COMMON PROGRAMMING CONCEPTS");
    // let x = 5;
    let mut x = 5;
    println!("the value of x is : {x}");
    x = 6;
    println!("the value of x is : {x}");

    // CONSTANTS
    // cant use mut with constant
    // always immutable
    const ONE_HOUR_IN_SECONDS: u32 = 1 * 60 * 60;
    println!("{ONE_HOUR_IN_SECONDS}");

    // SHADOWING
    println!("SHADOWING");
    let x = x + 1000; // SAHOWING
                      // you can declare a new variable with
                      // the same name as prev vars
    println!("{x}");
    // let mut x = x + 1000;
    // println!("{x}");
    let two_spaces = "  ";
    println!("{two_spaces}");
    let two_spaces = two_spaces.len();
    println!("{two_spaces}");
    let number = if 90 > 9 { 1000 } else { 2000 };
    loop {
        println!("LOOP");
        break;
    }
    // return a value from loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 100 {
            break counter;
            // returning value from loop
        }
    };
    println!("{result}");

    // loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        println!("count: {count}");
        let mut remaining = 10;
        loop {
            println!("remaining : {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count : {count}");
    let temp_arr = [1, 2, 3, 4, 5];
    for element in temp_arr {
        println!("value is : {element}");
    }
    for number in (1..5).rev() {
        println!("reverse : {number}");
    }
    let temperature = fahrenheit_to_celcius(129.0);
    println!("129 f = {temperature} c");
    let fib_num = fib_number_at(12);
    println!("fib num at 12 : {fib_num}");
}
