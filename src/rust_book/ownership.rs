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

pub fn ownership_intro() {
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
    let s2 = s1; // s2 point to s1
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
    // & :: REFERENCE
    // The &s1 syntax lets us create a reference
    // that refers to the value of s1 but does not
    // own it. Because it does not own it, the value
    // it points to will not be dropped when the
    // reference stops being used.
    //
    // Likewise, the signature of the function uses
    // & to indicate that the type of the parameter
    // s is a reference.

    //
    // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to s will fail:

    main();
    let mut s = String::from("MUTABLE STRING");
    let r1 = &mut s; // REF ONE
                     // let r2 = & mut s; // REF TWO
                     // This error says that this code is invalid because we cannot borrow s as mutable more than once at a time. The first mutable borrow is in r1 and must last until it’s used in the println!, but between the creation of that mutable reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.

    // WHY
    // -to prevent multiple pointer accessing the same
    // data at the same time
    // -At least one of the pointers is being used to write to the data.
    // -There’s no mechanism being used to synchronize access to the data.
    //

    // println!("true var:{} ref:{}", {s}, {r1});
    println!("ref1:{}", r1);
    let r2 = &mut s;
    println!("ref2 after we use ref1 {}", r2);

    // but u can use curly brackets
    {
        let r3 = &mut s;
    }
    let r4 = &mut s;
    // no error cause r3 is on its own scope

    // combingin mutable and immutable ref
    let r5 = &s; // immutable ref no problem
    let r6 = &s; // immutable ref no problem
                 // let r7 = &mut s; // mutabnle : ERROR PROBLEM
                 // if there is no mutable ref, its ok to have multiple immutable ref
    println!("{}, {}, and {}", r5, r6, 999);
    // println!("{}, {}, and {}", r5, r6, r7);

    // we dont expect cahnge when wes use immutable ref

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

    //
    // THE RULE OF REFERENCE
    // -At any given time, you can have either one mutable reference or any number of immutable references.
    // -References must always be valid.

    // THE SLICE TYPE
    // Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.
    println!("{}", first_word(&s));
    println!("{}", b' ');
    // We now have a straightforward API that’s much harder to mess up because the compiler will ensure the references into the String remain valid. Remember the bug in the program in Listing 4-8, when we got the index to the end of the first word but then cleared the string so our index was invalid? That code was logically incorrect but didn’t show any immediate errors. The problems would show up later if we kept trying to use the first word index with an emptied string. Slices make this bug impossible and let us know we have a problem with our code much sooner. Using the slice version of first_word will throw a compile-time error:
    // clear String
    // s.clear();

    // Rust disallows the mutable reference in clear and the immutable reference in word from existing at the same time, and compilation fails. Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!

    // STRING SLICES
    println!("the WORDS {}", s);
    // slice
    let mutable = &s[0..7];
    // or &s[..7];
    let string_2 = &s[8..s.len()];
    // s.clear();
    println!("slice one {};\nslice two {};", mutable, string_2);
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

    let my_string = String::from("hello world");

    // primitive str = immutable fixed-length string somewhere in memory
    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    //
}

///////////////////////////////////////////////////

// Note: The opposite of referencing by using & is
// dereferencing, which is accomplished with the
// dereference operator, *. We’ll see some uses of
// the dereference operator in Chapter 8 and discuss
// details of dereferencing in Chapter 15.
pub fn main() {
    let s1 = String::from("hello");

    // let (s2, len) = calculate_length(s1); // s2 borrow the value of s1
    let len = calculate_length(&s1);
    // & reference
    // A reference is like a pointer in that it’s an
    // address we can follow to access the data
    // stored at that address; that data is owned by
    // some other variable. Unlike a pointer,
    // a reference is guaranteed to point to a valid
    // value of a particular type for the life of
    // that reference.
    // println!("The length of '{}' is {}.", s2, len);
    println!("The length of '{}' is {}.", s1, len);

    // ERROR
    // `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
    // change(&s1);
    // not error
    let mut s4 = String::from("HELLO");
    println!("before change : {}", s4);
    change(&mut s4);
    println!("after cahnge: {}", s4);

    // DANGLING REFERENCE
    // let temp_s = dangle(); // error dangling reference, for more info goo look the fucntion below
    // let temp_s = no_dangle();
    //
}

pub fn calculate_length(s: &String) -> usize {
    // s is a reference to a Strin
    // borrowing : return it after we use
    // so what happen when we try to modify the var
    // that we borrow(reference)
    // pub fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
                          // (s, length)

    return length;
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// attempting to modify the reference var (borrow)
// pub fn change(some_string: &String){ // immutable
pub fn change(some_string: &mut String) {
    // mutable
    // some_string.push_str(", WORLD");
    // `some_string` is a `&` reference, so the data
    // it refers to cannot be borrowed as mutable
    //
    // SO REFERENCE IS IMMUTABLE BY DEFAULT

    // mutable
    some_string.push_str(", WORLD");
    // so above we clearly change variable value
    // we borrowed
}

// DANGLING REFERENCE
// pub fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

// NO DANGLE REF
pub fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

/////
/// THE SLICE TYPE
// pub fn first_word(s : &String) -> usize { // returning the index at the end of the first word
// pub fn first_word(s : &String) -> &str { // returning the index at the end of the first word
pub fn first_word(s: &str) -> &str {
    // returning the index at the end of the first word
    println!("SLICE TYPE : {}", s);
    let bytes = s.as_bytes();
    // println!("{} {}", s, bytes[0]);
    for (i, &item) in bytes.iter().enumerate() {
        // println!("{}", item); // b' ' bytes representation of " " (space)
        if item == b' ' {
            // return i;
            return &s[0..i];
        }
    }
    return &s[..];
    // return s.len(); // the input only one word
}

// CONCLUSION
// The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.

// Ownership affects how lots of other parts of Rust work, so we’ll talk about these concepts further throughout the rest of the book. Let’s move on to Chapter 5 and look at grouping pieces of data together in a struct.
