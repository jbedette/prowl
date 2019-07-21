use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Named {
    pub value: String,
}

impl Named {
    pub fn new(value: &str) -> Self {
        let value = String::from(value);
        Self { value }
    }

    pub fn name_or_noname(name: Option<&Named>) -> String {
        match name {
            Some(name) => name.value.clone(),
            None => String::from("NO_NAME")
        }
    }
}

impl Default for Named {
    fn default() -> Self {
        let value = String::from("NO NAME?");
        Self { value }
    }
}
