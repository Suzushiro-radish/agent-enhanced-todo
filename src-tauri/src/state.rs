use std::sync::Mutex;

use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub done: bool,
}

pub struct Todos {
    pub todos: Mutex<Vec<Todo>>,
}

impl Todos {
    pub fn new() -> Self {
        Self {
            todos: Mutex::new(vec![]),
        }
    }

    pub fn add(&self, name: String) {
        let mut todos = self.todos.lock().unwrap();
        let id = todos.len() as i32 + 1;
        todos.push(Todo {
            id,
            name,
            done: false,
        });
    }

    pub fn todo_list(&self) -> Vec<Todo> {
        let todos = self.todos.lock().unwrap();
        todos.clone()
    }
}
