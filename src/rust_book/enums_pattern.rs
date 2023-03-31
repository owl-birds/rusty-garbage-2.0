// Where structs give you a way of grouping together
// related fields and data, like a Rectangle with
// its width and height, enums give you a way of
// saying a value is one of a possible set of
// values. For example, we may want to say that
// Rectangle is one of a set of possible shapes that
// also includes Circle and Triangle. To do this,
// Rust allows us to encode these possibilities as
// an enum.

// Let’s look at a situation we might want to
// express in code and see why enums are useful and
// more appropriate than structs in this case. Say
// we need to work with IP addresses. Currently,
// two major standards are used for IP addresses:
// version four and version six. Because these are
// the only possibilities for an IP address that
// our program will come across, we can enumerate
// all possible variants, which is where enumeration
// gets its name.

use rand::{random, Rng};

// ip now have two kind of type v4 and v6
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}


pub fn main() {
    println!("ENUMS AND PATTERN");
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;
    dbg!(&ipv4); // remember dbg! take the ownership so we have to borrow it
    println!("{:#?}", ipv6);
    // Note that the variants of the enum are
    // namespaced under its identifier, and we use a
    // double colon to separate the two. This is
    // useful because now both values IpAddrKind::V4
    // and IpAddrKind::V6 are of the same type:
    // IpAddrKind. We can then, for instance, define
    // a function that takes any IpAddrKind:

    fn route(ip_addr: &IpAddrKind) {
        // boprrow not take ownership
        println!("{:#?}", ip_addr);
    }
    println!("ENUMS");
    route(&ipv4);
    route(&ipv6);
    route(&IpAddrKind::V4);
    route(&IpAddrKind::V6);

    // not recommended but you can do it
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // better one
    #[derive(Debug)]
    enum IpAddrKind2 {
        // V4(String),
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // enum can have differents types and amounts of
    // associated data

    // However, representing the same concept using
    // just an enum is more concise: rather than an
    // enum inside a struct, we can put data directly
    // into each enum variant. This new definition
    // of the IpAddr enum says that both V4 and V6
    // variants will have associated String values:
    // let ipv4_2 = IpAddrKind2::V4(String::from("127.0.0.1"));
    let ipv4_2 = IpAddrKind2::V4(127, 0, 0, 1);
    let ipv6_2 = IpAddrKind2::V6(String::from("::1"));
    println!("{:#?}", ipv4_2);
    println!("{:#?}", ipv6_2);

    // you can include many kind of data inside enum
    // even enum it self, struct etc

    #[derive(Debug)]
    struct Ipv4 {
        //
    }
    #[derive(Debug)]
    struct Ipv6 {
        //
    }
    #[derive(Debug)]
    enum IpAddrKind3 {
        V4(Ipv4),
        V6(Ipv6),
    }

    // Quit has no data associated with it at all.
    // Move has named fields, like a struct does.
    // Write includes a single String.
    // ChangeColor includes three i32 values.

    //
    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);
    struct ChangeColorMessage(i32, i32, i32);
    let quit_message = QuitMessage;
    // But if we used the different structs, each of
    // which has its own type, we couldn’t as easily
    // define a function to take any of these kinds \
    // of messages as we could with the Message enum
    // defined in Listing 6-2, which is a single type.

    // There is one more similarity between enums
    // and structs: just as we’re able to define
    // methods on structs using impl, we’re also
    // able to define methods on enums. Here’s a
    // method named call that we could define on our
    // Message enum:

    // sources : https://www.programiz.com/rust/enum
    // Enums (or enumerations) is a user-defined
    // data type that allows us to select a value
    // from a list of related values.
    enum Sport {
        Basketball,
        Volleyball,
        Football,
        Cricket,
    }
    // Here, we have created an enum named Sport
    // with a list of values Basketball, Volleyball,
    // Football and Cricket. These enum values are
    // known as variants.

    // Suppose you are creating a program where you
    // have to store a list of directions, and we
    // know that there will be only four possible
    // values for directions: North, East, West, and
    // Saouth.

    #[derive(Debug)]
    enum Directions {
        North,
        South,
        East,
        West,
    }

    let north = Directions::North;
    let east = Directions::East;
    let south = Directions::South;
    let west = Directions::West;

    println!("{:?}", north);
    println!("{:?}", east);
    println!("{:?}", south);
    println!("{:?}", west);

    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            //
            println!("{:?}", self);
        }
    }
    let msg = Message::Write(String::from("HELLO WORLD!"));
    println!("{:#?}", msg);
    msg.call();

    #[derive(Debug)]
    enum Animals {
        Dog(String, u32),
        Cat { name: String, age: u32 },
    }
    let mut dog_1 = Animals::Dog(String::from("Bob"), 7);
    let mut cat_1 = Animals::Cat {
        name: String::from("Oreo"),
        age: 3,
    };
    println!("{:?}", dog_1);
    dog_1 = Animals::Dog(String::from("Bob"), 3);
    println!("{:?}", dog_1);

    // null VALUE
    // The problem with null values is that if you
    // try to use a null value as a not-null value,
    // you’ll get an error of some kind. Because this
    // null or not-null property is pervasive, it’s
    // extremely easy to make this kind of error.

    // However, the concept that null is trying to
    // express is still a useful one: a null is a
    // value that is currently invalid or absent for
    // some reason.

    // The problem isn’t really with the concept but
    // with the particular implementation. As such,
    // Rust does not have nulls, but it does have an
    // enum that can encode the concept of a value
    // being present or absent. This enum is Option<T>,
    // and it is defined by the standard library as
    // follows:

    #[derive(Debug)]
    enum Option<T> {
        None,
        Some(T),
    }
    // The <T> syntax is a feature of Rust we haven’t
    //  talked about yet. It’s a generic type parameter,
    // and we’ll cover generics in more detail in
    // Chapter 10. For now, all you need to know
    // is that <T> means that the Some variant of the
    // Option enum can hold one piece of data of any
    // type, and that each concrete type that gets used
    // in place of T makes the overall Option<T> type
    // a different type. Here are some examples of
    // using Option values to hold number types
    // and string types:

    let some_number = Some(5);
    let some_name = Option::Some(String::from("Bob"));
    let some_float: Option<f32> = Option::Some(4.5);
    let absent_number: Option<i32> = Option::None;

    let x: i8 = 5;
    // let y_error: Option<i8> = Some(5); // ERROR HERE
    let y: Option<i8> = Option::Some(5);
    // error below couse of different types
    // let sum = x + y;

    // pattern matching
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => {
                println!("A QUARTER");
                25
            }
        }
    }
    // Let’s break down the match in the value_in_cents function. First we list the match keyword followed by an expression, which in this case is the value coin. This seems very similar to a conditional expression used with if, but there’s a big difference: with if, the condition needs to evaluate to a Boolean value, but here it can be any type. The type of coin in this example is the Coin enum that we defined on the first line.

    // Next are the match arms. An arm has two parts: a pattern and some code. The first arm here has a pattern that is the value Coin::Penny and then the => operator that separates the pattern and the code to run. The code in this case is just the value 1. Each arm is separated from the next with a comma.

    println!("Dime in cents : {}", value_in_cents(Coin::Dime));
    println!("Quarter in cents : {}", value_in_cents(Coin::Quarter));

    // contexts 1999 - 2008 : US minted quarters
    // with different design for each 50 states
    // only quarter got different design
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        //
    }
    enum Coin2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents_2(coin: Coin2) -> i32 {
        match coin {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("quarter at {:?} state", { state });
                25
            }
        }
    }
    println!(
        "Quarter value at Alaska : {}",
        value_in_cents_2(Coin2::Quarter(UsState::Alaska))
    );

    fn time_two(num: Option<i32>) -> Option<i32> {
        match num {
            Option::None => Option::None,
            Option::Some(n) => Option::Some(n * 2),
        }
    }
    println!("2 time 222 {:?}", time_two(Option::Some(222)));
    println!("2 time None {:?}", time_two(Option::None));
    println!("2 time 3 {:?}", time_two(Option::Some(3)));

    // Matches Are EXHAUSTIVE

    // below is not gonna compile, cause the code
    // not covering all the possibilities
    // the possibiolities in Option are Some and None

    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x { // not exhasutive
    //         Option::Some(i) => Option::Some(i + 1),
    //     }
    // }
    
    // let dice_roll
    let dice_roll: u32 = rand::thread_rng().gen_range(1..=6);
    match  dice_roll{
        1 => add_fancy_hat(),
        6 => remove_fancy_hat(),
        other => move_player(other)   
    }
    match  dice_roll{
        3 => add_fancy_hat(),
        4 => remove_fancy_hat(),
        _ => reroll()   
        // _ is a special pattern that matches any value 
        // and does not bind to that value. This tells 
        // Rust we aren’t going to use the value, so Rust 
        // won’t warn us about an unused variable.
    }
    fn add_fancy_hat(){println!("ADD FANCY HAT")}
    fn remove_fancy_hat(){println!("ADD FANCY HAT")}
    fn move_player(moved: u32){println!("moved : {moved}")}
    fn reroll(){println!("REROLL")}

    // CONCISE CONTROLL FLOW WITH if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => ()
    }
    if let Some(max) = config_max{
        println!("the maximum is configured to be {max}")
    }
}
