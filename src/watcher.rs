use notify::event::ModifyKind;
use notify::{Event, EventKind};
use std::fmt::{self, Display};
use std::path::Path;

/// Events broadcast to clients when files change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WatchEvent {
    /// File tree structure changed (file created or deleted)
    TreeChanged,
    /// A specific file's content changed
    FileChanged { path: String },
}

impl Display for WatchEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WatchEvent::TreeChanged => write!(f, "tree"),
            WatchEvent::FileChanged { path } => write!(f, "file:{}", path),
        }
    }
}

/// Process a notify event and return events to broadcast to clients.
pub fn process_event(event: &Event, preview_dir: &Path) -> Vec<WatchEvent> {
    // Ignore access events (open, close, read)
    if matches!(event.kind, EventKind::Access(_)) {
        return vec![];
    }

    let mut events = Vec::new();

    if matches!(
        event.kind,
        EventKind::Create(_) | EventKind::Remove(_) | EventKind::Modify(ModifyKind::Name(_))
    ) {
        events.push(WatchEvent::TreeChanged);
    }

    if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
        for path in &event.paths {
            if let Ok(relative_path) = path.strip_prefix(preview_dir) {
                events.push(WatchEvent::FileChanged {
                    path: relative_path.to_string_lossy().into_owned(),
                });
            }
        }
    }

    events
}

#[cfg(test)]
mod tests {
    use super::*;
    use notify::event::*;
    use std::path::PathBuf;

    fn make_event(kind: EventKind, paths: &[&str]) -> Event {
        Event {
            kind,
            paths: paths.iter().map(PathBuf::from).collect(),
            attrs: Default::default(),
        }
    }

    const PREVIEW_DIR: &str = "/preview";

    #[test]
    fn create_file_sends_tree_and_file() {
        let event = make_event(
            EventKind::Create(CreateKind::File),
            &["/preview/button.html"],
        );
        let events = process_event(&event, Path::new(PREVIEW_DIR));
        assert_eq!(
            events,
            vec![
                WatchEvent::TreeChanged,
                WatchEvent::FileChanged {
                    path: "button.html".into()
                },
            ]
        );
    }

    #[test]
    fn create_image_sends_tree_and_file() {
        let event = make_event(EventKind::Create(CreateKind::File), &["/preview/logo.svg"]);
        let events = process_event(&event, Path::new(PREVIEW_DIR));
        assert_eq!(
            events,
            vec![
                WatchEvent::TreeChanged,
                WatchEvent::FileChanged {
                    path: "logo.svg".into()
                },
            ]
        );
    }

    #[test]
    fn strips_nested_path() {
        let event = make_event(
            EventKind::Modify(ModifyKind::Data(DataChange::Any)),
            &["/preview/time/even-more/special.html"],
        );
        let events = process_event(&event, Path::new(PREVIEW_DIR));
        assert_eq!(
            events,
            vec![WatchEvent::FileChanged {
                path: "time/even-more/special.html".into()
            }]
        );
    }

    #[test]
    fn modify_image_sends_file_changed() {
        let event = make_event(
            EventKind::Modify(ModifyKind::Data(DataChange::Any)),
            &["/preview/images/photo.webp"],
        );
        let events = process_event(&event, Path::new(PREVIEW_DIR));
        assert_eq!(
            events,
            vec![WatchEvent::FileChanged {
                path: "images/photo.webp".into()
            }]
        );
    }

    #[test]
    fn rename_file_sends_tree_changed() {
        let event = make_event(
            EventKind::Modify(ModifyKind::Name(RenameMode::Any)),
            &["/preview/old.html", "/preview/new.html"],
        );
        let events = process_event(&event, Path::new(PREVIEW_DIR));
        assert!(events.contains(&WatchEvent::TreeChanged));
    }
}
