use clap::Parser;
use axum::{
    response::Html,
    extract::{Request, State},
    handler::HandlerWithoutStateExt,
    http::StatusCode,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use minijinja::{context, Environment, path_loader};
use tower::ServiceExt;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use state::AppState;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value_t=8000)]
    port: u16,

    #[arg(short, long, default_value_t=false)]
    debug: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let log_level = match args.debug{
        true => "info",
        false => "error",
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}={},tower_http={}", env!("CARGO_CRATE_NAME"), log_level, log_level).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // init templates
    let mut env = Environment::new();
    env.set_loader(path_loader("templates"));

    let app_state = Arc::new(AppState {env});

    tokio::join!(
        serve(get_router(app_state), args.port),
    );
}

fn get_router(app_state: Arc<AppState>) -> Router {
    // `ServeDir` allows setting a fallback if an asset is not found
    // so with this `GET /assets/doesnt-exist.jpg` will return `index.html`
    // rather than a 404
    let serve_dir = ServeDir::new("assets").not_found_service(ServeFile::new("assets/404.html"));
    Router::new()
        .route("/", get(routes::plain::home))
        .route("/writings", get(routes::blog::list))
        .route("/writings/:entry", get(routes::blog::get))
        .nest_service("/assets", serve_dir.clone())
        .with_state(app_state)
        .fallback_service(serve_dir)
}


async fn serve(app: Router, port: u16) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.layer(TraceLayer::new_for_http()))
        .await
        .unwrap();
}
