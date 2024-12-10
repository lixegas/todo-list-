use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::tasks::{list_tasks, create_task, update_task, delete_task};
use tower_http::cors::{Any, CorsLayer};
use axum::http::{header, Method};

pub fn tasks_routes() -> Router {
    
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE]) 
        .allow_headers(vec![header::CONTENT_TYPE]); 
    
    Router::new()
        .route("/tasks", get(list_tasks))
        .route("/create-task", post(create_task)) 
        .route("/update-task/:id", put(update_task)) 
        .route("/delete-task/:id", delete(delete_task)) 
        .layer(cors_layer) 
}
