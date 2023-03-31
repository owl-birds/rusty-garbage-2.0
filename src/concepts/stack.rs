use std::any::Any;

pub fn run() {
    println!("STACK RUNNING");
}

pub struct String_Stack {
    collections: Vec<String>,
    length: u64,
}

impl String_Stack {
    pub fn new() -> String_Stack {
        String_Stack {
            collections: vec![],
            length: 0,
        }
    }
    pub fn has(&self, object: &str) -> bool {
        for obj in self.collections.iter() {
            if obj == object {
                return true;
            }
        }
        return false;
    }
    pub fn insert(&mut self, object: &str) -> bool {
        self.collections.push(object.to_string());
        return true;
    }
    pub fn print_stack_collections(&self) {
        println!("{:?}", self.collections);
    }
}
