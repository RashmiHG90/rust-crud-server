use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use super::models::{CreateTask, Task, UpdateTask};
use super::db::TaskStore;


// Create a new task
async fn create_task(
    State(store): State<TaskStore>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, StatusCode> {
    let task = Task {
        id: Uuid::new_v4(),
        title: payload.title,
        description: payload.description,
        completed: false,
    };

    let mut tasks = store.write().await;
    tasks.insert(task.id, task.clone());
    
    Ok(Json(task))
}

// Get all tasks
async fn get_tasks(State(store): State<TaskStore>) -> Json<Vec<Task>> {
    let tasks = store.read().await;
    let task_list: Vec<Task> = tasks.values().cloned().collect();
    Json(task_list)
}

// Get a specific task
async fn get_task(
    Path(id): Path<Uuid>,
    State(store): State<TaskStore>,
) -> Result<Json<Task>, StatusCode> {
    let tasks = store.read().await;
    match tasks.get(&id) {
        Some(task) => Ok(Json(task.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Update a task
async fn update_task(
    Path(id): Path<Uuid>,
    State(store): State<TaskStore>,
    Json(payload): Json<UpdateTask>,
) -> Result<Json<Task>, StatusCode> {
    let mut tasks = store.write().await;
    match tasks.get_mut(&id) {
        Some(task) => {
            if let Some(title) = payload.title {
                task.title = title;
            }
            if let Some(description) = payload.description {
                task.description = Some(description);
            }
            if let Some(completed) = payload.completed {
                task.completed = completed;
            }
            Ok(Json(task.clone()))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Delete a task
async fn delete_task(
    Path(id): Path<Uuid>,
    State(store): State<TaskStore>,
) -> StatusCode {
    let mut tasks = store.write().await;
    match tasks.remove(&id) {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}
