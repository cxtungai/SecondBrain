//! Second Brain — một chương trình duy nhất:
//!   bấm đúp / `secondbrain`   → mở Bàn làm việc + xem trước web trên máy (tự cập nhật khi sửa file)
//!   `secondbrain build`        → xuất web tĩnh vào dist/ (GitHub Pages dùng lệnh này)
//!   `secondbrain check|new|publish|kb|seo` → lệnh cho Claude / người rành kỹ thuật

mod articles;
mod cli;
mod config;
mod kb;
mod mentions;
mod project;
mod publish;
mod render;
mod schema;
mod seo;
mod server;
mod site;
mod text;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = cli::run(args);
    std::process::exit(code);
}
