// Ownership is Rust’s most unique 
// feature and has deep implications 
// for the rest of the language. It 
// enables Rust to make memory safety 
// guarantees without needing a garbage 
// collector, so it’s important to 
// understand how ownership works. In 
// this chapter, we’ll talk about 
// ownership as well as several 
// related features: 
// borrowing, slices, and how Rust 
// lays data out in memory.

//
// Ownership is a set of rules that govern 
// how a Rust program manages memory.
//
// Rust uses a third approach: memory 
// is managed through a system of 
// ownership with a set of rules 
// that the compiler checks. If any 
// of the rules are violated, the 
// program won’t compile. None of the 
// features of ownership will slow 
// down your program while it’s running.
///
// ownership rules
// - Each value in rust has an owner.
// - there can only be one owner at a
//   time.
// - when the owner goes out of scope
//   , the value will be dropped.

pub fn ownership_intro(){
    println!("ownership");
    // VARIABLE SCOPE
    // {
    // no s (s not valid)
    // let s = "EXIST";
    // s valid 
    // do stuff with s
    // } scope is over, and s is no longer valid
    //
    // THE STRING TYPE
    // String type use heap
    let s = String::from("hello");
    // This type manages data 
    // allocated on the heap and as 
    // such is able to store an amount 
    // of text that is unknown to us 
    // at compile time. You can create 
    // a String from a string literal 
    // using the from function
    let mut s = String::from("GOOD");
    s.push_str(" MORNING!!!");
    println!("{}", s);
    // So, what’s the difference here? 
    // Why can String be mutated but 
    // literals cannot? The difference 
    // is in how these two types deal 
    // with memory.
    //
    // However, the second part is 
    // different. In languages with a 
    // garbage collector (GC), the GC 
    // keeps track of and cleans up 
    // memory that isn’t being used 
    // anymore, and we don’t need to 
    // think about it. In most 
    // languages without a GC, it’s 
    // our responsibility to identify 
    // when memory is no longer being 
    // used and to call code to 
    // explicitly free it, just as we 
    // did to request it. Doing this 
    // correctly has historically been 
    // a difficult programming problem. 
    // If we forget, we’ll waste memory. 
    // If we do it too early, we’ll 
    // have an invalid variable. If we 
    // do it twice, that’s a bug too. 
    // We need to pair exactly one 
    // allocate with exactly one free.
    //

    // Rust takes a different path: 
    // the memory is automatically 
    // returned once the variable that 
    // owns it goes out of scope.
        
    // There is a natural point at 
    // which we can return the memory 
    // our String needs to the 
    // allocator: when s goes out of 
    // scope. When a variable goes out 
    // of scope, Rust calls a special 
    // function for us. This function 
    // is called drop, and it’s where 
    // the author of String can put 
    // the code to return the memory. 
    // Rust calls drop automatically 
    // at the closing curly bracket.

    let x = 5;
    let y = x; //y make a copy of x
    // both x and y have the same value 
    // 5

    // now let see 
    let s1 = String::from("hello");
    let s2 = s1;// s2 point to s1
    let s3 = s2.clone();
    // not make a copy of s1
    // println!("{s1}"); // ERROR value borrowed here after move
    // BORROWED
    // so s1 was moved into s2
    println!("{s2}");
    println!("{s3}");
    // To ensure memory safety, after 
    // the line let s2 = s1;, Rust 
    // considers s1 as no longer 
    // valid. Therefore, Rust doesn’t 
    // need to free anything when s1 
    // goes out of scope. Check out 
    // what happens when you try to 
    // use s1 after s2 is created; it 
    // won’t work:

    // For more information about 
    // this error, try 
    // `rustc --explain E0382`.
    // error: could not compile `ownership`
    // due to previous error

    
    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

    // The ownership of a variable 
    // follows the same pattern every 
    // time: assigning a value to 
    // another variable moves it. 
    // When a variable that includes 
    // data on the heap goes out of 
    // scope, the value will be 
    // cleaned up by drop unless 
    // ownership of the data has been 
    // moved to another variable.

    // REFERENCES and BORROWING
}
pub fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

pub fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}