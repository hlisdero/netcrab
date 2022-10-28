pub enum NodeRef {
    Place(PlaceRef),
    Transition(TransitionRef),
}

pub struct PlaceRef {
    index: String,
}

pub struct TransitionRef {
    index: String,
}
