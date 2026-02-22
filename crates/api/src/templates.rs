use askama::Template;
use models::{Board, Thread, Post};

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;

#[derive(Template)]
#[template(path = "boards.html")]
pub struct BoardsTemplate {
    pub boards: Vec<Board>,
}

#[derive(Template)]
#[template(path = "board.html")]
pub struct BoardTemplate {
    pub board_id: String,
    pub board_name: String,
    pub threads: Vec<Thread>,
}

#[derive(Template)]
#[template(path = "thread.html")]
pub struct ThreadTemplate {
    pub thread: Thread,
    pub posts: Vec<Post>,
}

#[derive(Template)]
#[template(path = "catalog.html")]
pub struct CatalogTemplate {
    pub threads: Vec<Thread>,
}