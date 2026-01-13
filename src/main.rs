mod assets;
mod handlers;
mod models;
mod state;
mod templates;

use axum::{
    Router,
    routing::{get, post},
};
use handlers::{home, list, sse};
use state::AppContext;

const BIND: &'static str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    let ctx = AppContext::new().await;

    let app = Router::new()
        .route("/", get(home::home))
        .route("/new", get(home::new_list_form))
        .route("/create", post(home::create_list))
        .route("/list/:id", get(list::view_list))
        .route("/list/:id/add", post(list::add_item))
        .route("/list/:id/toggle/:idx", post(list::toggle_item))
        .route("/list/:id/edit/:idx", post(list::edit_item))
        .route("/list/:id/delete-item/:idx", post(list::delete_item))
        .route(
            "/list/:id/toggle-completed",
            post(list::toggle_show_completed),
        )
        .route(
            "/list/:id/delete-completed",
            post(list::delete_completed_items),
        )
        .route("/list/:id/sort", post(list::sort_list))
        .route("/list/:id/delete", post(list::delete_list))
        .route("/events", get(sse::sse_handler))
        .with_state(ctx);

    let listener = tokio::net::TcpListener::bind(BIND).await.unwrap();
    println!("Server running on http://{}", BIND);
    axum::serve(listener, app).await.unwrap();
}
