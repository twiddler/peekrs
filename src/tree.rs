use axum::{extract::State, response::Html};
use itertools::Itertools;
use std::{fs, path::Path};

use crate::AppState;

pub async fn handler(State(state): State<AppState>) -> Html<String> {
    Html(render_dir(&state.serve_dir, &state.serve_dir, 0))
}

fn render_dir(path: &Path, strip_prefix: &Path, depth: usize) -> String {
    let mut output = String::new();

    let mut entries = fs::read_dir(path)
        .into_iter()
        .flatten()
        .filter_map(Result::ok)
        .sorted_by_key(|e| e.file_name())
        .peekable();

    while let Some(entry) = entries.next() {
        let (connector, continuation) = if depth == 0 {
            ("", "")
        } else if entries.peek().is_none() {
            ("└ ", "  ")
        } else {
            ("├ ", "│ ")
        };

        let name = entry.file_name().to_string_lossy().into_owned();

        if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            output.push_str(&format!(
                "<span class=\"dim\">{connector}</span><span class=\"dir\">{name}/</span>\n"
            ));

            for line in render_dir(&entry.path(), strip_prefix, depth + 1).lines() {
                output.push_str(&format!(
                    "<span class=\"dim\">{continuation}</span>{line}\n"
                ));
            }
        } else {
            output.push_str(&format!(
                "<span class=\"dim\">{connector}</span><a id=\"{rel_path}\" href=\"/#{rel_path}\">{name}</a>\n",
                rel_path = entry.path().strip_prefix(strip_prefix).unwrap().display()
            ));
        }
    }

    output
}
