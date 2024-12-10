mod routes;
mod handlers;
mod db;
mod models;


#[tokio::main]
async fn main() {
    db::initialize_database();
    let app = routes::tasks::tasks_routes();

    println!("Server in ascolto su 127.0.0.1:3000");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}