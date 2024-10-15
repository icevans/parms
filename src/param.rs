pub struct Param {
    pub name: String,
    pub value: String,
}

impl Param {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}
