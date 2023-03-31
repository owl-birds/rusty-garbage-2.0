pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub code: char,
}

impl Color {
    pub fn details(&self) -> String {
        // return format!("RGB({}, {}, {})", self.red, self.green, self.blue);
        // format!("RGB({}, {}, {})", self.red, self.green, self.blue);
        let temp = format!("RGB({}, {}, {})", self.red, self.green, self.blue);
        return temp;
    }
}
