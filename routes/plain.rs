use std::sync::Arc;
use state::AppState;
use minijinja::context;
use axum::{
    response::Html,
    extract::State,
    http::StatusCode,
};

use types::Header;
use version::VERSION;

pub async fn home(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("index.html").unwrap();

    let rendered = template
        .render(context! {
            header => Header {
                title: String::from("Ian Kim"),
                subtitle: String::from("Engineer by day, broke philosopher by night."),
                author : String::from("Ian M Kim"),
                date: String::from("Sept 2 2024"),
                version: String::from(VERSION),
            }
        })
        .unwrap();

    Ok(Html(rendered))
}
