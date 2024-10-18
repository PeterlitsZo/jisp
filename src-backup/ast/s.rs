use super::item::Item;

#[derive(Debug, PartialEq, Eq, Clone)]
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

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn car(&self) -> Option<&Item> {
        self.items.first()
    }

    pub fn cdr(&self) -> Option<&[Item]> {
        if self.items.len() >= 1 { Some(&self.items[1..]) } else { None }
    }
}