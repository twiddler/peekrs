use axum::{
    extract::{Path, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use std::{fs, path};

use crate::AppState;

/// Reads requested files. Canonicalizes to resolve ".." segments and prevent path traversal attacks.
pub async fn handler(State(state): State<AppState>, Path(file_path): Path<String>) -> Response {
    let base = &*state.serve_dir;

    let Ok(full) = base.join(&file_path).canonicalize() else {
        return StatusCode::NOT_FOUND.into_response();
    };

    if !full.starts_with(base) {
        return StatusCode::NOT_FOUND.into_response();
    }

    match fs::read(&full) {
        Ok(content) => {
            let mime = mime_type(&full);
            ([(header::CONTENT_TYPE, mime)], content).into_response()
        }
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

fn mime_type(path: &path::Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("svg") => "image/svg+xml",
        Some("webp") => "image/webp",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        _ => "application/octet-stream",
    }
}
