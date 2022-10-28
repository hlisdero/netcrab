#[cfg(test)]
mod place_tests;

pub struct Place {
    pub label: Option<String>,
    pub marking: usize,
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

impl Default for Place {
    fn default() -> Self {
        Self {
            label: None,
            marking: 0,
        }
    }
}
