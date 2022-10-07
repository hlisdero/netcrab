#[cfg(test)]
mod transition_tests;

#[derive(Clone)]
pub struct Transition {
    pub label: Option<String>,
}

impl Transition {
    pub fn new(label: String) -> Transition {
        Transition { label: Some(label) }
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self { label: None }
    }
}
