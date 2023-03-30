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

// ip now have two kind of type v4 and v6
#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6
}

pub fn main(){
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

    fn route(ip_addr: &IpAddrKind){ // boprrow not take ownership
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
    enum IpAddrKind2{
        // V4(String),
        V4(u8, u8, u8, u8),
        V6(String)
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
}