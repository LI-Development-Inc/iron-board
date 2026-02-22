use axum::{
    routing::{get, post},
    Router,
    extract::{State, Form, Path},
    response::{Html, IntoResponse},
    http::StatusCode,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use services::ServiceLayer;
use storage::{thread_repository, post_repository};
use templates::*;

#[derive(Clone)]
pub struct AppState {
    pub services: Arc<ServiceLayer>,
    pub db: Arc<rusqlite::Connection>,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(index))
        .route("/boards", get(list_boards))
        .route("/boards/:id", get(view_board))
        .route("/threads/:id", get(view_thread))
        .route("/threads", post(create_thread))
        .route("/posts", post(create_post))
        .with_state(state)
}

async fn index() -> impl IntoResponse {
    let template = IndexTemplate;
    Html(template.render().unwrap())
}

async fn list_boards(
    State(state): State<AppState>,
) -> Result<Html<String>, StatusCode> {
    let boards = state
        .services
        .list_boards(&state.db)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let template = BoardsTemplate { boards };

    Ok(Html(template.render().unwrap()))
}

#[derive(Deserialize)]
struct CreateThreadForm {
    board_id: String,
    title: String,
}

async fn create_thread(
    State(state): State<AppState>,
    Form(form): Form<CreateThreadForm>,
) -> Result<impl IntoResponse, StatusCode> {
    let board_id =
        Uuid::parse_str(&form.board_id)
            .map_err(|_| StatusCode::BAD_REQUEST)?;

    state
        .services
        .create_thread(&state.db, board_id, form.title)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::SEE_OTHER)
}

#[derive(Deserialize)]
struct CreatePostForm {
    thread_id: String,
    content: String,
}

async fn create_post(
    State(state): State<AppState>,
    Form(form): Form<CreatePostForm>,
) -> Result<impl IntoResponse, StatusCode> {
    let thread_id =
        Uuid::parse_str(&form.thread_id)
            .map_err(|_| StatusCode::BAD_REQUEST)?;

    state
        .services
        .create_post(&state.db, thread_id, form.content)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::SEE_OTHER)
}

async fn view_thread(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let thread_id =
        Uuid::parse_str(&id)
            .map_err(|_| StatusCode::BAD_REQUEST)?;

    let posts =
        post_repository::get_posts_by_thread(&state.db, thread_id)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let thread = models::Thread {
        id: thread_id,
        board_id: Uuid::nil(),
        title: "Thread".into(),
        created_at: time::OffsetDateTime::now_utc(),
    };

    let template = ThreadTemplate { thread, posts };

    Ok(Html(template.render().unwrap()))
}

async fn view_board(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Html<String>, StatusCode> {
    let board_id =
        Uuid::parse_str(&id)
            .map_err(|_| StatusCode::BAD_REQUEST)?;

    let threads =
        thread_repository::get_threads_by_board(&state.db, board_id)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let template = BoardTemplate {
        board_id: id,
        board_name: "Board".into(),
        threads,
    };

    Ok(Html(template.render().unwrap()))
}




/// TESTS:
/// 
/// 
/// 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn router_builds() {
        let state = AppState {
            services: Arc::new(ServiceLayer),
            db: Arc::new(rusqlite::Connection::open_in_memory().unwrap()),
        };

        let _router = create_router(state);
    }
}