use super::test::super_keyword_test as super_ina_test;
use crate::concepts::stack::run as stack_run;

pub fn run(){
    println!("TEST DIRECTORY");
    println!("Trying using module in file that isnt main.rs");
    // crate::concepts::stack::run(); 
    stack_run();
    // super::test::super_keyword_test();
    super_ina_test();
    // so we start at the parent path/root 
    //https://www.sheshbabu.com/posts/rust-module-system/
}
pub fn super_keyword_test(){
    println!("TEST super");
}
// We can call it by using the fully qualified name 
// crate::routes::health_route::print_health_route() 
// but we can also use a relative path 
// super::health_route::print_health_route();. 
// Notice that we’ve used super to refer to the 
// parent scope.