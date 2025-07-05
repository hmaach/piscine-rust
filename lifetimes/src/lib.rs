#[derive(Debug)]
pub struct Person<'a> {
    pub name: &'a str,
    pub age: u8,
}

impl<'a> Person<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name, age: 0 }
    }
}
