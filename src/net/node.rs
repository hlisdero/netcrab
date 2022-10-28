#[cfg(test)]
mod place_tests;
#[cfg(test)]
mod transition_tests;

pub struct Place {
    pub label: Option<String>,
    pub marking: usize,
}

pub struct Transition {
    pub label: Option<String>,
}

impl Place {
    pub fn new(label: String) -> Place {
        Place {
            label: Some(label),
            marking: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.marking == 0
    }

    pub fn add_token(&mut self) {
        self.marking += 1
    }

    pub fn remove_token(&mut self) -> Result<(), &str> {
        if self.is_empty() {
            return Err("Cannot remove token from empty place");
        }
        self.marking -= 1;
        Ok(())
    }
}

impl Transition {
    pub fn new(label: String) -> Transition {
        Transition { label: Some(label) }
    }
}

impl Default for Place {
    fn default() -> Self {
        Self {
            label: None,
            marking: 0,
        }
    }
}

impl Default for Transition {
    fn default() -> Self {
        Self { label: None }
    }
}
