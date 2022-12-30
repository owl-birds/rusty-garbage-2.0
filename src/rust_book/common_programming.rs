pub fn common_progamming_concepts(){
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
    let two_spaces= "  ";
    println!("{two_spaces}");
    let two_spaces = two_spaces.len();
    println!("{two_spaces}");
    let number = if 90 > 9 {1000} else {2000};
    loop {
        println!("LOOP");
        break;
    }
    // return a value from loop
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 100{
            break counter; 
            // returning value from loop
        }
    };
    println!("{result}");
    // loop labels to disambiguate between multiple loops
}