use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const TRAIL_FILE: &str = "trail_directories.txt";

pub fn find_src_path(path: &Path) -> Option<PathBuf> {
    let mut curr_path = path;
    while curr_path.exists() {
        if curr_path.join("Cargo.toml").exists() {
            return Some(curr_path.join("src"));
        }
        if let Some(parent) = curr_path.parent() {
            curr_path = parent;
        } else {
            break;
        }
    }
    None
}

pub fn find_rs_files(src_path: &Path) -> Vec<PathBuf> {
    let mut rs_file_paths: Vec<PathBuf> = Vec::new();
    for entry in WalkDir::new(src_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
            rs_file_paths.push(path.to_path_buf());
        }
    }
    rs_file_paths
}

pub fn create_trail_file() -> PathBuf {
    let home_dir = env::var("HOME").expect("Failed to get home directory");
    let trail_file_path = Path::new(&home_dir).join(TRAIL_FILE);
    println!("{}", trail_file_path.display());
    trail_file_path
}

pub fn read_file_content(path: &Path) -> io::Result<String> {
    fs::read_to_string(path)
}

pub fn filter_todo_lines(content: &str) -> Vec<(usize, &str)> {
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| line.contains("TODO") || line.contains("todo!()"))
        .collect()
}

pub fn generate_buffer_content(path: &Path, result: &[(usize, &str)], count: &mut usize) -> String {
    let mut buffer = String::new();
    for (line_no, line) in result {
        writeln!(
            &mut buffer,
            "{}. {}, line number {}, located at {}",
            *count,
            line.trim(),
            line_no + 1,
            path.display()
        )
        .expect("Error writing to buffer");
        *count += 1;
    }
    buffer
}

pub fn write_to_file(file_path: &Path, content: &str) -> io::Result<()> {
    fs::write(file_path, content)?;
    Ok(())
}
