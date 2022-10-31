#[derive(Clone, Eq, Hash, PartialEq)]
pub struct PlaceRef(pub String);

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct TransitionRef(pub String);

impl PlaceRef {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl TransitionRef {
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }
}
