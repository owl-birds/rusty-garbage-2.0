mod big_o;
mod concepts;
mod rust_book;
mod test_directory;
mod training;

fn main() {
    // rust book tutorial : https://doc.rust-lang.org/book/foreword.html
    // rust_book::intro::intro_hello();
    // rust_book::intro::guess_the_number();
    // rust_book::common_programming::common_progamming_concepts();
    //
    // rust_book::ownership::ownership_intro();
    //
    // rust_book::intro_struct::main();
    //
    rust_book::enums_pattern::main();
    /////////////////
    /////////////////
    /////////////////
    // println!("Hello, world!");
    // big_o::run();
    // println!("Pointer");
    // training::run();
    // let user1 = training::person::Person::new("Johnm", 21, "janitor");
    // user1.not_secret();
    // let color1 = training::test::Color{
    //     red: 255,
    //     green: 255,
    //     blue: 255,
    //     code: 'A',
    // };
    // println!("{}", color1.details());
    // let mut temp_string_stack_1 = concepts::stack::String_Stack::new();
    // temp_string_stack_1.insert("user1");
    // temp_string_stack_1.insert("user2");
    // println!("is user1 is in the stack : {}", temp_string_stack_1.has("user1"));
    // println!("is user2 is in the stack : {}", temp_string_stack_1.has("user2"));
    // println!("is user3 is in the stack : {}", temp_string_stack_1.has("user3"));
    // temp_string_stack_1.print_stack_collections();
    // concepts::stack::run();
    // crate::concepts::stack::run();
    // u can do it like this too
    // test_directory::test::run();
}
