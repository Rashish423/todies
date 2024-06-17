use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub text: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(text: String) -> Todo {
        Todo {
            text,
            completed: false,
        }
    }
}
