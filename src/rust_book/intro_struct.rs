struct User{
    is_active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// TUPLE STRUCT 
struct RGB_color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit like struct without any fields

fn build_user(username: String, email: String)->User{
    User{
        username, // username: username
        email,
        is_active: false,
        sign_in_count: 1
    }
}

pub fn main(){
    println!("intro Struct");
    // creating an instance of User 
    let mut user1 = User{
        is_active: false,
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        sign_in_count: 0
    };

    // rust doesnt allow us to make a specific fields a mutable, so u have to make the instane
    // mutable first, if u want to change the value
    // changing value
    user1.username =  String::from("user_1");
    let mut user2 = build_user(String::from("user2"), String::from("user2@gmail.com"));
    let mut user3 = User{
        username: String::from("user3"),
        email: String::from("user3@gmail.com"),
        ..user2
    };
    // Note that the struct update syntax uses = like an assignment; this is 
    // because it moves the data, just as we saw in the “Variables and Data 
    // Interacting with Move” section. In this example, we can no longer use user1 as a whole 
    // after creating user2 because the String in the username field of user1 was moved into 
    // user2. If we had given user2 new String values for both email and username, and 
    // thus only used the active and sign_in_count values from user1, then user1 would 
    // still be valid after creating user2. Both active and sign_in_count are types that 
    // implement the Copy trait, so the behavior we discussed in the “Stack-Only Data: Copy” section would apply.
    

    // ..user1 must come last to specify that any remaining fields should get their values from the corresponding fields in user1 
    println!("{}\n{}\n{}\n{}", user3.username, user3.email, user3.is_active, user3.sign_in_count);
    
    let user4 = User{..user2}; // can no longer use user2, because borrow and ownership, it moved
                              // from user2 to user 4
   let user5 = User{
       username: String::from("user5"),
       ..user4
   };
    // println!("{}", user2.username);
    println!("{}", user4.username);
    

    let white = RGB_color(255,255,255);
    let origin = Point(0, 0, 0); 
    // Each struct you define is its own type, even though the fields within the struct 
    // might have the same types. For example, a function that takes a parameter of type 
    // Color cannot take a Point as an argument, even though both types are made up of 
    // three i32 values. Otherwise, tuple struct instances are similar to tuples in 
    // that you can destructure them into their individual pieces, and you can use a . 
    // followed by the index to access an individual value.

}
