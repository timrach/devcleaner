use std::path::Path;
use walkdir::{DirEntry, WalkDir};

pub mod app;
pub mod components;
pub mod crossterm;
pub mod ui;

// These are the clutter folders I found on my disk when writing the tool.
// They might correspond to multiple languages or to folders that contain
// other files than development dependencies. For example, a folder 'obj'
// could also contain 3D files in the obj format instead of XCode build files.
static DEV_DEPS: &[&str] = &["node_modules", "target", "build", "dist", "obj", "venv"];

pub fn is_dev_dep(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| DEV_DEPS.contains(&s))
        .unwrap_or(false)
}

pub fn move_to_bin(path: &Path) -> Result<(), trash::Error> {
    trash::delete(path)
}

pub fn scan_dir(dir: &Path) -> Vec<String> {
    if !dir.exists() {
        panic!("search path does not exist")
    }

    let mut result: Vec<String> = vec![];
    let mut it = WalkDir::new(dir).into_iter();

    while let Some(Ok(entry)) = it.next() {
        if entry.file_type().is_dir() && is_dev_dep(&entry) {
            let e = entry.path().to_str().unwrap();
            result.push(String::from(e));
            it.skip_current_dir();
            continue;
        }
    }
    result
}
