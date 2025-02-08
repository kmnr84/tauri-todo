use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Todo {
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(title: String) -> Result<Self, &'static str> {
        if title.is_empty() {
            Err("title cannot be empty")
        } else {
            Ok(Self {
                title,
                completed: false,
            })
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
    }
}