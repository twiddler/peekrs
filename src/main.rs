mod args;
mod raw;
mod shell;
mod tree;
mod watcher;
mod websocket;

use args::Args;
use axum::{Router, routing::get};
use clap::Parser;
use notify::{Event, RecursiveMode, Watcher};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::broadcast;
use watcher::WatchEvent;

#[derive(Clone)]
pub struct AppState {
    pub tx: broadcast::Sender<WatchEvent>,
    pub serve_dir: Arc<PathBuf>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (tx, _rx) = broadcast::channel::<WatchEvent>(16);

    let _watcher = create_watcher(tx.clone(), args.dir.clone()).unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    eprintln!("Serving http://{} …", addr);

    let app = create_router(tx, args.dir);
    axum::serve(listener, app).await.unwrap();
}

fn create_watcher(
    tx: broadcast::Sender<WatchEvent>,
    serve_dir: PathBuf,
) -> Result<impl Watcher, notify::Error> {
    let serve_dir_clone = serve_dir.clone();

    let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
        if let Ok(event) = res {
            for event in watcher::process_event(&event, &serve_dir_clone) {
                match &event {
                    WatchEvent::TreeChanged => eprintln!("Tree changed, notifying clients …"),
                    WatchEvent::FileChanged { path } => {
                        eprintln!("File changed: {}, notifying clients …", path)
                    }
                }
                let _ = tx.send(event);
            }
        }
    })?;

    watcher.watch(&serve_dir, RecursiveMode::Recursive)?;
    eprintln!("Watching {} for changes …", serve_dir.display());

    Ok(watcher)
}

fn create_router(tx: broadcast::Sender<WatchEvent>, serve_dir: PathBuf) -> Router {
    let state = AppState {
        tx,
        serve_dir: Arc::new(serve_dir),
    };

    Router::new()
        .route("/", get(shell::handler))
        .route("/tree", get(tree::handler))
        .route("/ws", get(websocket::handler))
        .route("/raw/{*path}", get(raw::handler))
        .with_state(state)
}
