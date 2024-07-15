use rocket::serde::{Serialize, Deserialize};



#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Todo{
    pub id: usize,
    pub title: String,
    pub completed: bool,
}

use std::sync::Mutex;

pub struct TodoList {
    pub todos: Mutex<Vec<Todo>>,
}