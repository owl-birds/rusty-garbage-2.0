struct User{
    is_active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// TUPLE STRUCT 
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit like struct without any fields
struct AlwaysEqual;

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
    

    let white = Color(255,255,255);
    let origin = Point(0, 0, 0); 
    let black = Color(0,0,0);

    // black != origin :: eventhough it has the same value

    // Each struct you define is its own type, even though the fields within the struct 
    // might have the same types. For example, a function that takes a parameter of type 
    // Color cannot take a Point as an argument, even though both types are made up of 
    // three i32 values. Otherwise, tuple struct instances are similar to tuples in 
    // that you can destructure them into their individual pieces, and you can use a . 
    // followed by the index to access an individual value.

    // unit like structs without any fields
    let subject = AlwaysEqual;


    // example program using Structs
    
    // area of rectangle
    fn rect_area_1(width: u32, height: u32)->u32{
        width * height
    }
    let width_1 = 40;
    let height_1 = 50;
    println!(
        "AREA OF RECT: {} square units", 
        rect_area_1(width_1, height_1)
    );

    // better one
    fn rect_area_2(dimensions: (u32, u32))->u32{
        dimensions.0 * dimensions.1
    }
    let dimensions_1 = (30, 40);
    println!(
        "AREA OF RECT: {} square units", 
        rect_area_2(dimensions_1)
    );

    // using strucy :: we re adding more meaning and readablility
    struct Rectangle{
        height: u32,
        width: u32
    }
    fn rect_area_3(dimensions: &Rectangle)->u32{
        dimensions.height * dimensions.width
    }
    let dimensions_2 = Rectangle{
        width: 20,
        height: 50
    };
    println!(
        "AREA OF RECT: {} square units", 
        rect_area_3(&dimensions_2)
    );
    // without borrowing :: recomemeded to borrow struct, 
    // so main can retains its ownership and can continue 
    // using dimesnions_2, which is why we use the & in 
    // in the function signature
    fn rect_area_4(dimensions: Rectangle)->u32{
        dimensions.height * dimensions.width
    } // not recommended, we need to borrow the value no t move it
    // println!(
    //     "AREA OF RECT: {} square units", 
    //     rect_area_4(dimensions_2)
    // );
    // println!("{}", dimensions_2.width); // error here if we dont borrow it in the functions

    // we cant do this
    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // println!("rectangle 2 : {}", dimensions_2); // ERROR

    // Rust does include functionality to print out 
    // debugging information, but we have to 
    // explicitly opt in to make that functionality 
    // available for our struct. To do that, 
    // we add the outer attribute #[derive(Debug)] 
    // just before the struct definition, as shown 
    // in Listing 5-12.
    
    #[derive(Debug)] // 
    struct RectangleTwo{
        width: u32,
        height: u32
    }
    let rect_1 = RectangleTwo{width:40, height:20};
    println!(
        "reactangle with Debug functionality: {:?}", 
        rect_1
    );
    println!(
        "reactangle with Debug functionality: {:#?}", 
        rect_1
    );
    // Another way to print out a value using the 
    // Debug format is to use the dbg! macro, which 
    // takes ownership of an expression (as opposed 
    // to println!, which takes a reference), prints 
    // the file and line number of where that dbg! 
    // macro call occurs in your code along with the 
    // resultant value of that expression, and 
    // returns ownership of the value.
    let scale = 2;
    let rect_2 = RectangleTwo {
        width: dbg!(30 * scale),
        height: 50,
    };
    println!(
        "println reactangle with Debug functionality: {:#?}", 
        rect_2
    ); // println takes references
     /// dbg! take the ownership
    dbg!(&rect_2); // we need to use references
    // dbg!(rect_2);
    // println!(
    //     "reactangle with Debug functionality: {:#?}", 
    //     rect_2
    // );

    // We can see the first bit of output came 
    // from src/main.rs line 10 where we’re 
    // debugging the expression 30 * scale, 
    // and its resultant value is 60 (the Debug 
    // formatting implemented for integers is 
    // to print only their value). The dbg! call 
    // on line 14 of src/main.rs outputs the value of 
    // &rect1, which is the Rectangle struct. 
    // This output uses the pretty Debug formatting 
    // of the Rectangle type. The dbg! macro can be 
    // really helpful when you’re trying to figure 
    // out what your code is doing!

    // defining methods in struct
    #[derive(Debug)]
    struct RectangleThree{
        width: u32,
        height: u32
    }
    impl RectangleThree {
        // Associated Functions
        fn square(size: u32)->Self{
            Self { width: size, height: size }
        }
        fn rectangle(width: u32, height: u32)->Self{
            Self { width: width, height: height }
        }

        fn area(&self)->u32{ 
            // we want to borrwo the struct here 
            // not to take the ownership

            // using just self as the first parameter is 
            // rare; this technique is usually used when 
            // the method transforms self into something 
            // else and you want to prevent the caller from 
            // using the original instance after the 
            // transformation.
            self.width * self.height
        }
        fn circumferences(&self)->u32{
            (2*self.width)+(2*self.height)
        } 
        // SETTERS or change or whatever
        fn change_width(&mut self, new_width: u32){
            self.width = new_width;
        }  
        fn change_height(&mut self, new_height: u32){
            self.height = new_height;
        }
        // getters
        fn get_width(&self)->u32{self.width}
        fn get_height(&self)->u32{self.height}
        
        // u can have the same name properties and 
        // methods
        fn width(&self)->bool{
            if self.width > 0{return true}
            false
        }
        fn can_hold(&self, other_rect: &RectangleThree)->bool{
            self.width >= other_rect.width 
            && self.height >= other_rect.height
        }
    }
    let mut rect_3 = RectangleThree{
        width: 10,
        height: 70
    };
    println!("STRUCT WITH METHODS");
    dbg!(&rect_3);
    println!("rect 3 area : {}", rect_3.area());
    println!("rect 3 circumferences : {}", rect_3.circumferences());
    rect_3.change_height(90);
    rect_3.change_width(20);
    println!("CAHNGE TO NEW VALUE width and height");
    dbg!(&rect_3);
    println!("rect 3 area : {}", rect_3.area());
    println!("rect 3 circumferences : {}", rect_3.circumferences());

    // 
    println!("width properties : {}", rect_3.width);
    println!("width methods : {}", rect_3.width());

    let rect_4 = RectangleThree{
        width: 20,
        height: 40
    };
    let rect_5 = RectangleThree{
        width: 30,
        height: 10
    };
    println!("can rect 3 hold rect 4 : {}", rect_3.can_hold(&rect_4));
    println!("can rect 3 hold rect 5 : {}", rect_3.can_hold(&rect_5));

    // Associated Functions
    // All functions defined within an impl block are 
    // called associated functions because they’re 
    // associated with the type named after the 
    // impl. We can define associated functions 
    // that don’t have self as their first parameter 
    // (and thus are not methods) because they don’t 
    // need an instance of the type to work with. 
    // We’ve already used one function like this: 
    // the String::from function that’s defined on 
    // the String type.
    
    // Associated functions that aren’t methods are 
    // often used for constructors that will return 
    // a new instance of the struct.

    // we can have muiltiple impl BLOCKS
    impl RectangleThree{
        fn multiple_impl(&self){
            println!("MULTIPLE IMPL");
        }
    }

    let square_1 = RectangleThree::square(10);
    let rectangle_1 = RectangleThree::rectangle(90, 100);
    println!("square_1 : {:#?},\n area : {}", square_1, square_1.area());
    println!("rectangle_1 : {:#?},\n area : {}", rectangle_1, rectangle_1.area());

    square_1.multiple_impl();
    rectangle_1.multiple_impl();
}

// SUMMARY
// Structs let you create custom types that are 
// meaningful for your domain. By using structs, 
// you can keep associated pieces of data connected 
// to each other and name each piece to make your 
// code clear. In impl blocks, you can define 
// functions that are associated with your 
// type, and methods are a kind of associated 
// function that let you specify the behavior that 
// instances of your structs have.

// But structs aren’t the only way you can create 
// custom types: let’s turn to Rust’s enum feature 
// to add another tool to your toolbox.