use rocket::serde::json::Json;
use rocket::State;
use crate::models::{Todo, TodoList};


#[get("/")]
pub fn list_todos(state: State<TodoList>) -> Json<Vec<Todo>>{
    let todos = state.todos.lock().unwrap();
    Json(todos.clone())
}