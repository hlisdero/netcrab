#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PlaceRef(pub String);

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct TransitionRef(pub String);

impl PlaceRef {
    pub fn as_string(&self) -> &String {
        &self.0
    }
}

impl TransitionRef {
    pub fn as_string(&self) -> &String {
        &self.0
    }
}
