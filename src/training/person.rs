pub struct Person{
    name: String,
    pub age: u8,
    pub occupation: String,
}

impl Person {
    // construct
    pub fn new(insert_name: &str, insert_age:u8, insert_occupation: &str) ->Person{
        Person{
            name: insert_name.to_string(),
            age: insert_age,
            occupation : insert_occupation.to_string(),
        } 
    }
    fn secret_cant_be_accessed_in_other_mods(&self){
        println!("SIKE");
    }
    pub fn not_secret(&self){
        println!("NOT SECRET");
    }
}