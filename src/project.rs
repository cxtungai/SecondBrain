//! Tìm thư mục dự án và đọc toàn bộ ghi chú + bài viết từ đĩa.

use crate::articles::Articles;
use crate::config::Config;
use crate::kb::{FileEntry, Kb};
use std::path::{Path, PathBuf};

pub struct Project {
    pub root: PathBuf,
}

pub struct Loaded {
    pub config: Config,
    pub config_error: Option<String>,
    pub kb: Kb,
    pub articles: Articles,
}

impl Project {
    /// Thư mục dự án = nơi có thư mục `knowledge/`: thư mục hiện tại, thư mục chứa file chạy, hoặc thư mục cha của chúng.
    pub fn find() -> Result<Project, String> {
        let mut starts: Vec<PathBuf> = Vec::new();
        if let Ok(cwd) = std::env::current_dir() {
            starts.push(cwd);
        }
        if let Ok(exe) = std::env::current_exe() {
            if let Some(dir) = exe.parent() {
                starts.push(dir.to_path_buf());
            }
        }
        for start in &starts {
            let mut dir: Option<&Path> = Some(start);
            while let Some(d) = dir {
                if d.join("knowledge").is_dir() {
                    return Ok(Project { root: d.to_path_buf() });
                }
                dir = d.parent();
            }
        }
        Err("Không tìm thấy thư mục dự án (cần có thư mục knowledge/). Hãy đặt chương trình trong thư mục SecondBrain.".into())
    }

    pub fn abs(&self, rel: &str) -> PathBuf {
        self.root.join(rel)
    }

    /// Đường dẫn tương đối dạng "knowledge/x/y.md".
    pub fn rel(&self, abs: &Path) -> String {
        abs.strip_prefix(&self.root).unwrap_or(abs).to_string_lossy().replace('\\', "/")
    }

    pub fn read_md(&self, dir: &str) -> Vec<FileEntry> {
        let mut out = Vec::new();
        walk(&self.root.join(dir), &mut |p| {
            if p.extension().is_some_and(|e| e == "md") {
                if let Ok(raw) = std::fs::read_to_string(p) {
                    out.push(FileEntry { path: self.rel(p), raw });
                }
            }
        });
        out
    }

    pub fn load(&self, include_drafts: bool) -> Loaded {
        let (config, config_error) = Config::load(&self.root);
        let kb = Kb::build(&self.read_md("knowledge"), include_drafts);
        let articles = Articles::build(&self.read_md("articles"), &kb, include_drafts);
        Loaded { config, config_error, kb, articles }
    }

    /// Dấu vân tay nội dung (số file + thời điểm sửa) để biết khi nào cần dựng lại.
    pub fn fingerprint(&self) -> u128 {
        let mut acc: u128 = 0;
        for dir in ["knowledge", "articles"] {
            walk(&self.root.join(dir), &mut |p| {
                if let Ok(meta) = std::fs::metadata(p) {
                    let t = meta.modified().ok().and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_nanos()).unwrap_or(0);
                    acc = acc.wrapping_mul(31).wrapping_add(t).wrapping_add(meta.len() as u128).wrapping_add(p.as_os_str().len() as u128);
                }
            });
        }
        if let Ok(meta) = std::fs::metadata(self.root.join(crate::config::CONFIG_FILE)) {
            let t = meta.modified().ok().and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok()).map(|d| d.as_nanos()).unwrap_or(0);
            acc = acc.wrapping_add(t);
        }
        acc
    }
}

fn walk(dir: &Path, f: &mut dyn FnMut(&Path)) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    let mut paths: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    paths.sort();
    for p in paths {
        if p.is_dir() {
            walk(&p, f);
        } else {
            f(&p);
        }
    }
}
