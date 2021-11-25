struct Leaf{
    value : String
}

impl Leaf{
    pub fn new(value : String) -> {
        Leaf {
            value: value
        }
    },
    pub fn get_value(&self) -> String {
        &self.value
    }
}