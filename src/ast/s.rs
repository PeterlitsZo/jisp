use super::item::Item;

#[derive(Debug, PartialEq, Eq)]
pub struct S {
    items: Vec<Item>,
}

impl S {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn from<T>(value: T) -> Self where T: Into<Vec<Item>> {
        Self { items: value.into() }
    }

    pub fn push_item(&mut self, item: Item) {
        self.items.push(item);
    }
}