//! "Hợp đồng" cấu trúc ghi chú: loại, trạng thái, thang điểm uy tín, đọc frontmatter.

use crate::text::is_iso_date;
use regex::Regex;
use serde::Serialize;
use serde_yaml::{Mapping, Value};
use std::sync::LazyLock;

pub struct TypeInfo {
    pub key: &'static str,
    pub label: &'static str,
    pub folder: &'static str,
    pub description: &'static str,
}

pub const NOTE_TYPES: &[TypeInfo] = &[
    TypeInfo { key: "topic", label: "Chủ đề", folder: "topics", description: "Ghi chú tổng hợp (bản đồ nội dung) dẫn tới các ghi chú liên quan trong một mảng kiến thức." },
    TypeInfo { key: "concept", label: "Khái niệm", folder: "concepts", description: "Định nghĩa, giải thích một khái niệm, thuật ngữ hoặc nguyên lý." },
    TypeInfo { key: "fact", label: "Số liệu & dữ kiện", folder: "facts", description: "Số liệu, thống kê, mốc thời gian, dữ kiện có thể trích dẫn. Bắt buộc có nguồn." },
    TypeInfo { key: "framework", label: "Mô hình & phương pháp", folder: "frameworks", description: "Mô hình, quy trình, phương pháp, checklist có thể áp dụng." },
    TypeInfo { key: "case-study", label: "Case study", folder: "case-studies", description: "Ví dụ thực tế: bối cảnh, cách làm, kết quả và bài học." },
    TypeInfo { key: "source", label: "Nguồn tham khảo", folder: "sources", description: "Tóm tắt một cuốn sách, bài báo, báo cáo, video… kèm các ý chính." },
];

pub fn type_info(key: &str) -> Option<&'static TypeInfo> {
    NOTE_TYPES.iter().find(|t| t.key == key)
}

pub fn type_label(key: &str) -> &'static str {
    type_info(key).map(|t| t.label).unwrap_or("Ghi chú")
}

pub struct StatusInfo {
    pub key: &'static str,
    pub label: &'static str,
    pub description: &'static str,
}

pub const STATUSES: &[StatusInfo] = &[
    StatusInfo { key: "draft", label: "Bản nháp", description: "Chưa kiểm chứng — AI chỉ dùng làm gợi ý, không trích dẫn như sự thật." },
    StatusInfo { key: "verified", label: "Đã kiểm chứng", description: "Đã đối chiếu nguồn — AI có thể dùng và trích dẫn." },
    StatusInfo { key: "outdated", label: "Lỗi thời", description: "Thông tin cũ — AI không được dùng, trừ khi nói về bối cảnh lịch sử." },
];

pub fn status_info(key: &str) -> &'static StatusInfo {
    STATUSES.iter().find(|s| s.key == key).unwrap_or(&STATUSES[0])
}

/// Thang điểm uy tín nguồn 1–5: (điểm, nhãn, mô tả).
pub const CREDIBILITY: &[(u8, &str, &str)] = &[
    (5, "Rất cao", "Nguồn gốc hoặc chính thức: nghiên cứu có bình duyệt, cơ quan quản lý, sở giao dịch, dữ liệu sơ cấp, tài liệu gốc của người đặt ra khái niệm."),
    (4, "Cao", "Sách chuyên môn của tác giả có uy tín, báo chí lớn, tổ chức nghề nghiệp/đào tạo có tên tuổi — có biên tập và dẫn nguồn rõ ràng."),
    (3, "Trung bình", "Chuyên gia/người thực hành có danh tiếng và thành tích kiểm chứng được; trang kiến thức chuyên ngành có biên tập."),
    (2, "Thấp", "Blog cá nhân, video, khoá học chưa rõ kiểm chứng; ý kiến cá nhân ít dẫn chứng."),
    (1, "Rất thấp", "Diễn đàn, mạng xã hội, nguồn ẩn danh, nội dung quảng cáo hoặc có xung đột lợi ích rõ ràng."),
];

pub fn credibility_info(score: u8) -> Option<(&'static str, &'static str)> {
    CREDIBILITY.iter().find(|c| c.0 == score).map(|c| (c.1, c.2))
}

pub const SOURCE_KINDS: &[(&str, &str)] = &[
    ("paper", "Nghiên cứu khoa học"),
    ("official", "Cơ quan / tổ chức chính thức"),
    ("data", "Dữ liệu sơ cấp"),
    ("book", "Sách"),
    ("media", "Báo chí"),
    ("education", "Trang kiến thức / đào tạo"),
    ("expert", "Chuyên gia / người thực hành"),
    ("blog", "Blog"),
    ("video", "Video / podcast"),
    ("forum", "Diễn đàn / mạng xã hội"),
    ("other", "Khác"),
];

pub fn source_kind_label(key: &str) -> Option<&'static str> {
    SOURCE_KINDS.iter().find(|k| k.0 == key).map(|k| k.1)
}

pub const KNOWN_FIELDS: &[&str] =
    &["title", "type", "summary", "tags", "status", "sources", "aliases", "created", "updated", "reviewed", "research"];

#[derive(Debug, Clone, Serialize)]
pub struct Source {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credibility: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credibility_note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

// ---------- Đọc frontmatter ----------

static FRONTMATTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?s)\A\x{feff}?---\r?\n(.*?)\r?\n---[ \t]*(?:\r?\n|\z)").unwrap());

/// Tách frontmatter YAML và nội dung.
pub fn split_frontmatter(raw: &str) -> (Option<&str>, &str) {
    match FRONTMATTER.captures(raw) {
        Some(c) => (Some(c.get(1).unwrap().as_str()), &raw[c.get(0).unwrap().end()..]),
        None => (None, raw),
    }
}

pub fn parse_yaml_map(yaml: &str) -> Result<Mapping, String> {
    match serde_yaml::from_str::<Value>(yaml) {
        Ok(Value::Mapping(m)) => Ok(m),
        Ok(Value::Null) => Ok(Mapping::new()),
        Ok(_) => Err("Frontmatter phải là danh sách key: value".into()),
        Err(e) => Err(format!("YAML lỗi: {}", e.to_string().lines().next().unwrap_or(""))),
    }
}

/// Giá trị đơn → chuỗi; null hoặc "" → None.
pub fn scalar_string(v: &Value) -> Option<String> {
    let s = match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Tagged(t) => return scalar_string(&t.value),
        _ => return None,
    };
    let t = s.trim();
    if t.is_empty() {
        None
    } else {
        Some(t.to_string())
    }
}

pub fn get<'a>(map: &'a Mapping, key: &str) -> Option<&'a Value> {
    map.get(Value::String(key.to_string()))
}

pub fn get_str(map: &Mapping, key: &str) -> Option<String> {
    get(map, key).and_then(scalar_string)
}

/// Danh sách chuỗi: chấp nhận `[a, b]`, `a, b`, bỏ dấu # đầu, bỏ trùng.
pub fn string_list(v: Option<&Value>) -> Vec<String> {
    let raw: Vec<String> = match v {
        None | Some(Value::Null) => vec![],
        Some(Value::Sequence(seq)) => seq.iter().filter_map(scalar_string).collect(),
        Some(other) => scalar_string(other).map(|s| s.split(',').map(String::from).collect()).unwrap_or_default(),
    };
    let mut out: Vec<String> = Vec::new();
    for item in raw {
        let t = item.trim().trim_start_matches('#').trim().to_string();
        if !t.is_empty() && !out.contains(&t) {
            out.push(t);
        }
    }
    out
}

/// Trường ngày YYYY-MM-DD; ghi lỗi nếu sai định dạng hoặc thiếu khi bắt buộc.
pub fn date_field(map: &Mapping, key: &str, required: bool, errors: &mut Vec<String>) -> Option<String> {
    match get_str(map, key) {
        Some(s) if is_iso_date(&s) => Some(s),
        Some(_) => {
            errors.push(format!("{key}: Ngày phải có dạng YYYY-MM-DD"));
            None
        }
        None => {
            if required {
                errors.push(format!("{key}: Thiếu ngày (YYYY-MM-DD)"));
            }
            None
        }
    }
}

/// Đọc danh sách nguồn; bỏ qua khối để trống từ template.
pub fn parse_sources(v: Option<&Value>, errors: &mut Vec<String>) -> Vec<Source> {
    let seq = match v {
        Some(Value::Sequence(s)) => s.clone(),
        None | Some(Value::Null) => return vec![],
        Some(other) => match scalar_string(other) {
            Some(_) => vec![other.clone()],
            None => return vec![],
        },
    };
    let mut out = Vec::new();
    for (i, item) in seq.iter().enumerate() {
        let src = match item {
            Value::String(s) => {
                let s = s.trim();
                if s.is_empty() {
                    continue;
                }
                if s.starts_with("http://") || s.starts_with("https://") {
                    Source::bare(s, Some(s))
                } else {
                    Source::bare(s, None)
                }
            }
            Value::Mapping(m) => {
                let title = get_str(m, "title");
                let url = get_str(m, "url");
                if title.is_none() && url.is_none() {
                    continue; // khối nguồn trống của template
                }
                let Some(title) = title else {
                    errors.push(format!("sources.{i}.title: Nguồn cần có title"));
                    continue;
                };
                if let Some(u) = &url {
                    if !(u.starts_with("http://") || u.starts_with("https://")) {
                        errors.push(format!("sources.{i}.url: url không hợp lệ (phải bắt đầu bằng http:// hoặc https://)"));
                    }
                }
                let kind = get_str(m, "kind");
                if let Some(k) = &kind {
                    if source_kind_label(k).is_none() {
                        let keys: Vec<&str> = SOURCE_KINDS.iter().map(|k| k.0).collect();
                        errors.push(format!("sources.{i}.kind: kind phải là một trong: {}", keys.join(", ")));
                    }
                }
                let credibility = match get_str(m, "credibility") {
                    None => None,
                    Some(s) => match s.parse::<u8>() {
                        Ok(n) if (1..=5).contains(&n) => Some(n),
                        _ => {
                            errors.push(format!("sources.{i}.credibility: credibility từ 1 đến 5"));
                            None
                        }
                    },
                };
                let accessed = get_str(m, "accessed");
                if let Some(a) = &accessed {
                    if !is_iso_date(a) {
                        errors.push(format!("sources.{i}.accessed: Ngày phải có dạng YYYY-MM-DD"));
                    }
                }
                Source {
                    title,
                    url,
                    author: get_str(m, "author"),
                    publisher: get_str(m, "publisher"),
                    date: get_str(m, "date"),
                    accessed,
                    kind,
                    credibility,
                    credibility_note: get_str(m, "credibility_note"),
                    note: get_str(m, "note"),
                }
            }
            _ => continue,
        };
        out.push(src);
    }
    out
}

impl Source {
    fn bare(title: &str, url: Option<&str>) -> Self {
        Source {
            title: title.to_string(),
            url: url.map(String::from),
            author: None,
            publisher: None,
            date: None,
            accessed: None,
            kind: None,
            credibility: None,
            credibility_note: None,
            note: None,
        }
    }
}

/// YAML → JSON (để giữ trường lạ khi xuất dữ liệu).
pub fn yaml_to_json(v: &Value) -> serde_json::Value {
    serde_json::to_value(v).unwrap_or(serde_json::Value::Null)
}
