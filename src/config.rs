//! Cấu hình site trong file cau-hinh.yml (tên, mô tả, tác giả, miễn trừ, quy tắc cho AI).

use crate::schema::{self, get, get_str, scalar_string};
use serde::Serialize;
use serde_yaml::Value;
use std::path::Path;

pub const CONFIG_FILE: &str = "cau-hinh.yml";

#[derive(Debug, Clone, Serialize)]
pub struct Config {
    pub name: String,
    pub description: String,
    pub language: String,
    pub locale: String,
    pub author: String,
    pub disclaimer: String,
    pub agent_rules: Vec<String>,
    /// Mã xác minh Google Search Console (thẻ meta "google-site-verification") — trống nếu chưa đăng ký.
    pub google_site_verification: String,
    /// Mã xác minh Bing Webmaster Tools (thẻ meta "msvalidate.01") — trống nếu chưa đăng ký.
    pub bing_site_verification: String,
    /// Khoá IndexNow (8–128 ký tự chữ, số, "-"): web phục vụ `<khoá>.txt`, GitHub dùng khoá này báo
    /// Bing, Yandex, Naver, Seznam mỗi khi có trang mới/sửa. Khoá công khai theo thiết kế, không phải mật khẩu.
    pub indexnow_key: String,
}

/// Khoá IndexNow hợp lệ: 8–128 ký tự a-z, A-Z, 0-9, "-".
pub fn valid_indexnow_key(k: &str) -> bool {
    (8..=128).contains(&k.len()) && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

/// Nhận cả mã trơn lẫn nguyên thẻ `<meta name=… content="mã" />` người dùng dán vào.
pub fn verification_code(raw: &str) -> String {
    let s = raw.trim();
    if let Some(i) = s.find("content=") {
        let rest = &s[i + 8..];
        let q = rest.chars().next().filter(|c| *c == '"' || *c == '\'');
        let rest = if q.is_some() { &rest[1..] } else { rest };
        let end = rest.find(|c: char| Some(c) == q || (q.is_none() && (c.is_whitespace() || c == '/' || c == '>'))).unwrap_or(rest.len());
        return rest[..end].trim().to_string();
    }
    s.chars().filter(|c| c.is_ascii_alphanumeric() || "-_=+.".contains(*c)).collect()
}

impl Default for Config {
    fn default() -> Self {
        Config {
            name: "Second Brain".into(),
            description: "Kho tri thức chuyên môn được chắt lọc và kiểm chứng — để người đọc tra cứu nhanh và để AI agent viết content chính xác, có nguồn.".into(),
            language: "vi".into(),
            locale: "vi_VN".into(),
            author: String::new(),
            disclaimer: "Nội dung nhằm mục đích tham khảo và học tập, không phải lời khuyên đầu tư, tài chính hay chuyên môn cho trường hợp cụ thể của bạn.".into(),
            agent_rules: vec![
                "Chỉ dùng số liệu, dữ kiện, trích dẫn từ ghi chú có status \"verified\" như sự thật; luôn ghi nguồn theo trường sources của ghi chú đó.".into(),
                "Ghi chú \"draft\" chỉ dùng làm ý tưởng hoặc định hướng — không trích dẫn con số hay khẳng định từ đó như sự thật.".into(),
                "Không dùng ghi chú \"outdated\" cho content mới, trừ khi nói về bối cảnh lịch sử.".into(),
                "Không bịa số liệu, nguồn hay trích dẫn. Nếu kho không có thông tin cần thiết, nói rõ là thiếu và đề xuất ghi chú cần bổ sung.".into(),
                "Ưu tiên ghi chú cập nhật gần hơn khi có thông tin mâu thuẫn, và nêu rõ mốc thời gian của số liệu.".into(),
                "Bắt đầu từ ghi chú loại \"topic\" (bản đồ chủ đề) để nắm tổng quan, rồi đi theo liên kết tới ghi chú chi tiết.".into(),
            ],
            google_site_verification: String::new(),
            bing_site_verification: String::new(),
            indexnow_key: String::new(),
        }
    }
}

impl Config {
    /// Đọc cau-hinh.yml; thiếu file hoặc thiếu trường thì dùng mặc định.
    pub fn load(root: &Path) -> (Config, Option<String>) {
        let mut cfg = Config::default();
        let Ok(raw) = std::fs::read_to_string(root.join(CONFIG_FILE)) else {
            return (cfg, None);
        };
        let map = match schema::parse_yaml_map(&raw) {
            Ok(m) => m,
            Err(e) => return (cfg, Some(format!("{CONFIG_FILE}: {e}"))),
        };
        let set = |field: &mut String, key: &str| {
            if let Some(v) = get(&map, key) {
                *field = scalar_string(v).unwrap_or_default();
            }
        };
        set(&mut cfg.description, "description");
        set(&mut cfg.author, "author");
        set(&mut cfg.disclaimer, "disclaimer");
        set(&mut cfg.google_site_verification, "google_site_verification");
        set(&mut cfg.bing_site_verification, "bing_site_verification");
        cfg.google_site_verification = verification_code(&cfg.google_site_verification);
        cfg.bing_site_verification = verification_code(&cfg.bing_site_verification);
        set(&mut cfg.indexnow_key, "indexnow_key");
        cfg.indexnow_key = cfg.indexnow_key.trim().to_string();
        if !valid_indexnow_key(&cfg.indexnow_key) {
            cfg.indexnow_key.clear();
        }
        if let Some(v) = get_str(&map, "name") {
            cfg.name = v;
        }
        if let Some(v) = get_str(&map, "language") {
            cfg.language = v;
        }
        if let Some(v) = get_str(&map, "locale") {
            cfg.locale = v;
        }
        if let Some(Value::Sequence(rules)) = get(&map, "agent_rules") {
            let rules: Vec<String> = rules.iter().filter_map(scalar_string).collect();
            if !rules.is_empty() {
                cfg.agent_rules = rules;
            }
        }
        (cfg, None)
    }
}
