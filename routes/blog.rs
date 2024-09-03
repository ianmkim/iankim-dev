use std::io;
use std::io::{Read};
use std::fs::File;
use std::io::BufRead;
use std::sync::Arc;

use walkdir::WalkDir;
use serde::Serialize;

use state::AppState;
use minijinja::context;
use axum::{
    response::Html,
    extract::{State, Path},
    http::StatusCode,
};

use types::{Header, Writing};

fn read_writing(file_path: String) -> Result<Writing, io::Error> {
    let file = File::open(file_path)?;
    let mut reader = io::BufReader::new(file);
    let mut header_str = vec![
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
    ];
    for i in 0..5 {
        reader.read_line(&mut header_str[i])?;
    }

    let mut delimiter = String::new();
    reader.read_line(&mut delimiter);

    let mut header = Header {
        title: header_str[0].clone(),
        subtitle: header_str[1].clone(),
        author: header_str[2].clone(),
        date: header_str[3].clone(),
        version: header_str[4].clone(),
    };

    let mut raw_content = String::new();
    reader.read_to_string(&mut raw_content)?;
    let html = markdown::to_html_with_options(&raw_content, &markdown::Options::gfm()).unwrap();

    Ok(Writing {
        header,
        content: html,
    })
}

#[derive(Serialize)]
struct Preview {
    title: String,
    link: String,
}

fn read_writings_dir(dir_path: &std::path::Path) -> Vec<Preview> {
    let mut previews = Vec::new();
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        if let Some(file_path) = entry.path().to_str() {
            if file_path != dir_path.to_str().unwrap(){
                previews.push(Preview {
                    title: file_path.to_string(),
                    link: file_path.to_string(),
                });
            }
        }
    }
    previews
}

pub async fn list(State(state): State<Arc<AppState>>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("writings.html").unwrap();

    let stories = read_writings_dir(std::path::Path::new("writings"));

    let rendered = template
        .render(context! { writings => stories })
        .unwrap();

    Ok(Html(rendered))
}

pub async fn get(State(state): State<Arc<AppState>>, Path(entry): Path<String>) -> Result<Html<String>, StatusCode> {
    let template = state.env.get_template("writing.html").map_err(|_| StatusCode::NOT_FOUND)?;
    let story = read_writing(format!("writings/{}", entry)).map_err(|_| StatusCode::NOT_FOUND)?;

    let rendered = template
        .render(context! {
            header => story.header,
            content => story.content,
        })
        .unwrap();

    Ok(Html(rendered))
}
