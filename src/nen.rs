//! Vẽ hình nến minh hoạ (SVG) từ khối ```nen — chỉ dùng cho trang web, không lưu ảnh.
//!
//! Cú pháp, mỗi dòng một lệnh:
//! - `ten: Tiêu đề hình`                  chú thích dưới hình
//! - `mở cao thấp đóng`                   một cây nến, vd. `100 108 97 105`
//! - `mở cao thấp đóng | nhãn`            nến được tô nổi, kèm nhãn (tự đặt trên hoặc dưới nến)
//! - `-`                                  một chỗ trống giữa các nến
//! - `ngang giá nhãn`                     đường ngang nét đứt, vd. `ngang 98 Hỗ trợ`
//! - `vung giá1 giá2 nhãn`                vùng giá tô nhạt (cả chiều ngang)
//! - `vung giá1 giá2 nhãn @3-8`           vùng giá chỉ từ cây nến thứ 3 tới thứ 8, nhãn đặt trên vùng
//! - `giai-phau`                          ghi tên các phần (thân, bóng, giá mở/đóng/cao/thấp) cho nến đầu tiên
//!
//! Giá là số thường (`57300`, `97.5`); hiển thị theo kiểu Việt Nam (`57.300`, `97,5`).

use crate::text::escape_html;

#[derive(Debug, Clone, PartialEq)]
struct Candle {
    o: f64,
    h: f64,
    l: f64,
    c: f64,
    label: Option<String>,
}

#[derive(Debug)]
struct Zone {
    lo: f64,
    hi: f64,
    label: String,
    /// Khoảng cây nến (đếm từ 1), `None` = cả chiều ngang.
    span: Option<(usize, usize)>,
}

#[derive(Debug, Default)]
struct Figure {
    title: Option<String>,
    slots: Vec<Option<Candle>>,
    lines: Vec<(f64, String)>,
    zones: Vec<Zone>,
    anatomy: bool,
}

fn num(s: &str) -> Option<f64> {
    s.trim().replace(',', ".").parse::<f64>().ok().filter(|v| v.is_finite())
}

fn parse(src: &str) -> Result<Figure, String> {
    let mut f = Figure::default();
    for (i, raw) in src.lines().enumerate() {
        let line = raw.trim();
        let at = |msg: &str| format!("dòng {}: {msg} (\"{line}\")", i + 1);
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(t) = line.strip_prefix("ten:").or_else(|| line.strip_prefix("tên:")) {
            f.title = Some(t.trim().to_string());
            continue;
        }
        if line == "giai-phau" || line == "giải-phẫu" {
            f.anatomy = true;
            continue;
        }
        if line == "-" {
            f.slots.push(None);
            continue;
        }
        let mut words = line.split_whitespace();
        let first = words.next().unwrap_or("");
        if first == "ngang" {
            let p = words.next().and_then(num).ok_or_else(|| at("thiếu giá cho đường ngang"))?;
            f.lines.push((p, words.collect::<Vec<_>>().join(" ")));
            continue;
        }
        if first == "vung" || first == "vùng" {
            let a = words.next().and_then(num).ok_or_else(|| at("thiếu giá thứ nhất của vùng"))?;
            let b = words.next().and_then(num).ok_or_else(|| at("thiếu giá thứ hai của vùng"))?;
            let mut rest: Vec<&str> = words.collect();
            let span = match rest.last().and_then(|w| w.strip_prefix('@')) {
                Some(r) => {
                    let (i, j) = r.split_once('-').unwrap_or((r, r));
                    let (i, j) = (i.parse::<usize>().map_err(|_| at("khoảng nến phải dạng @3-8"))?, j.parse::<usize>().map_err(|_| at("khoảng nến phải dạng @3-8"))?);
                    if i == 0 || j < i {
                        return Err(at("khoảng nến phải dạng @3-8, đếm từ 1"));
                    }
                    rest.pop();
                    Some((i, j))
                }
                None => None,
            };
            f.zones.push(Zone { lo: a.min(b), hi: a.max(b), label: rest.join(" "), span });
            continue;
        }
        let (nums, label) = match line.split_once('|') {
            Some((n, l)) => (n, Some(l.trim().to_string())),
            None => (line, None),
        };
        let v: Vec<f64> = nums.split_whitespace().map(num).collect::<Option<Vec<_>>>().ok_or_else(|| at("không đọc được số"))?;
        let [o, h, l, c] = v[..] else { return Err(at("một cây nến cần đủ 4 số: mở cao thấp đóng")) };
        if h < o.max(c) || l > o.min(c) || h < l {
            return Err(at("giá cao nhất phải ≥ giá mở và đóng, giá thấp nhất phải ≤ giá mở và đóng"));
        }
        f.slots.push(Some(Candle { o, h, l, c, label: label.filter(|s| !s.is_empty()) }));
    }
    if f.slots.iter().all(|s| s.is_none()) {
        return Err("khối nến không có cây nến nào".into());
    }
    Ok(f)
}

/// Hiển thị giá kiểu Việt Nam: 57300 → "57.300", 97.5 → "97,5".
pub fn fmt_price(v: f64) -> String {
    let neg = v < 0.0;
    let v = v.abs();
    let int = v.trunc() as u64;
    let frac = ((v - v.trunc()) * 100.0).round() as u64;
    let digits = int.to_string();
    let mut grouped = String::new();
    for (i, ch) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            grouped.push('.');
        }
        grouped.push(ch);
    }
    let mut s = if neg { format!("-{grouped}") } else { grouped };
    if frac > 0 {
        let f = format!("{frac:02}");
        s.push(',');
        s.push_str(f.trim_end_matches('0'));
    }
    s
}

/// Độ rộng ước tính (px) của chữ cỡ 11.
fn text_w(s: &str) -> f64 {
    s.chars().count() as f64 * 6.2
}

/// Vẽ khối ```nen thành `<figure>` chứa SVG. Lỗi cú pháp trả về `Err` để báo cho người viết.
pub fn render(src: &str) -> Result<String, String> {
    let f = parse(src)?;
    let slot = if f.anatomy { 64.0 } else { 26.0 };
    let body_w = if f.anatomy { 30.0 } else { 13.0 };
    let plot_h = if f.anatomy { 210.0 } else { 170.0 };
    let (top, bottom) = (24.0, 26.0);

    let candles: Vec<&Candle> = f.slots.iter().flatten().collect();
    let mut hi = candles.iter().map(|c| c.h).fold(f64::MIN, f64::max);
    let mut lo = candles.iter().map(|c| c.l).fold(f64::MAX, f64::min);
    for (p, _) in &f.lines {
        hi = hi.max(*p);
        lo = lo.min(*p);
    }
    for z in &f.zones {
        hi = hi.max(z.hi);
        lo = lo.min(z.lo);
    }
    let pad = if hi > lo { (hi - lo) * 0.06 } else { 1.0 };
    let (hi, lo) = (hi + pad, lo - pad);

    let side_labels: Vec<String> = f.lines.iter().map(|(p, t)| format!("{t} {}", fmt_price(*p))).chain(f.zones.iter().filter(|z| z.span.is_none()).map(|z| z.label.clone())).collect();
    let right = if f.anatomy { 150.0 } else { 10.0 + side_labels.iter().map(|s| text_w(s)).fold(0.0, f64::max) };
    let left = if f.anatomy { 78.0 } else { 10.0 };
    let plot_w = (f.slots.len() as f64 * slot).max(if f.anatomy { slot } else { 120.0 });
    let width = left + plot_w + right;
    let height = top + plot_h + bottom;
    let y = |p: f64| top + (hi - p) / (hi - lo) * plot_h;
    let x0 = left + (plot_w - f.slots.len() as f64 * slot) / 2.0;
    let cx = |i: usize| x0 + slot * (i as f64 + 0.5);

    let mut svg = String::new();
    // Vùng giá (phía sau cùng).
    for z in &f.zones {
        let (zx, zw) = match z.span {
            Some((i, j)) => (x0 + slot * (i - 1) as f64, slot * (j.min(f.slots.len()) + 1 - i.min(j)) as f64),
            None => (left, plot_w),
        };
        svg += &format!(r#"<rect class="nen-vung" x="{zx:.1}" y="{:.1}" width="{zw:.1}" height="{:.1}"/>"#, y(z.hi), (y(z.lo) - y(z.hi)).max(1.0));
        if !z.label.is_empty() {
            let t = escape_html(&z.label);
            svg += &match z.span {
                Some(_) => format!(r#"<text class="nen-chu" x="{:.1}" y="{:.1}" text-anchor="middle">{t}</text>"#, zx + zw / 2.0, y(z.lo) + 14.0),
                None => format!(r#"<text class="nen-chu" x="{:.1}" y="{:.1}">{t}</text>"#, left + plot_w + 6.0, (y(z.lo) + y(z.hi)) / 2.0 + 4.0),
            };
        }
    }
    // Nền tô nổi cho nến có nhãn.
    for (i, s) in f.slots.iter().enumerate() {
        if let Some(c) = s.as_ref().filter(|c| c.label.is_some() && !f.anatomy) {
            svg += &format!(r#"<rect class="nen-noi" x="{:.1}" y="{:.1}" width="{:.1}" height="{:.1}" rx="4"/>"#, cx(i) - slot / 2.0 + 1.0, y(c.h) - 5.0, slot - 2.0, y(c.l) - y(c.h) + 10.0);
        }
    }
    // Đường ngang.
    for (p, t) in &f.lines {
        let yy = y(*p);
        svg += &format!(r#"<line class="nen-ngang" x1="{left:.1}" y1="{yy:.1}" x2="{:.1}" y2="{yy:.1}"/>"#, left + plot_w);
        svg += &format!(r#"<text class="nen-chu" x="{:.1}" y="{:.1}">{} <tspan class="nen-gia">{}</tspan></text>"#, left + plot_w + 6.0, yy + 4.0, escape_html(t), fmt_price(*p));
    }
    // Nến.
    for (i, s) in f.slots.iter().enumerate() {
        let Some(c) = s else { continue };
        let kind = if c.c > c.o { "nen-tang" } else if c.c < c.o { "nen-giam" } else { "nen-doji" };
        let x = cx(i);
        svg += &format!(r#"<line class="nen-bong {kind}" x1="{x:.1}" y1="{:.1}" x2="{x:.1}" y2="{:.1}"/>"#, y(c.h), y(c.l));
        let (bt, bb) = (y(c.o.max(c.c)), y(c.o.min(c.c)));
        let bh = (bb - bt).max(2.0);
        svg += &format!(r#"<rect class="nen-than {kind}" x="{:.1}" y="{:.1}" width="{body_w:.1}" height="{bh:.1}"/>"#, x - body_w / 2.0, bt - if bb - bt < 2.0 { 1.0 } else { 0.0 });
        if let Some(t) = c.label.as_ref().filter(|_| !f.anatomy) {
            let below = (c.h + c.l) / 2.0 < (hi + lo) / 2.0;
            let ty = if below { y(c.l) + 18.0 } else { y(c.h) - 9.0 };
            let half = text_w(t) / 2.0;
            let (tx, anchor) = if x - half < 2.0 { (x - slot / 2.0 + 2.0, "start") } else if x + half > width - 2.0 { (x + slot / 2.0 - 2.0, "end") } else { (x, "middle") };
            svg += &format!(r#"<text class="nen-nhan" x="{tx:.1}" y="{ty:.1}" text-anchor="{anchor}">{}</text>"#, escape_html(t));
        }
    }
    // Giải phẫu: ghi tên các phần của nến đầu tiên.
    if f.anatomy {
        if let Some((i, c)) = f.slots.iter().enumerate().find_map(|(i, s)| s.as_ref().map(|c| (i, c))) {
            let x = cx(i);
            let up = c.c >= c.o;
            let (top_name, bot_name) = if up { ("Giá đóng cửa", "Giá mở cửa") } else { ("Giá mở cửa", "Giá đóng cửa") };
            let rx = x + body_w / 2.0 + 10.0;
            for (p, name) in [(c.h, "Giá cao nhất"), (c.o.max(c.c), top_name), (c.o.min(c.c), bot_name), (c.l, "Giá thấp nhất")] {
                let yy = y(p);
                svg += &format!(r#"<line class="nen-chi" x1="{:.1}" y1="{yy:.1}" x2="{rx:.1}" y2="{yy:.1}"/>"#, x + 2.0);
                svg += &format!(r#"<text class="nen-chu" x="{:.1}" y="{:.1}">{name} <tspan class="nen-gia">{}</tspan></text>"#, rx + 4.0, yy + 4.0, fmt_price(p));
            }
            let lx = x - body_w / 2.0 - 12.0;
            let parts = [(c.h, c.o.max(c.c), "Bóng trên"), (c.o.max(c.c), c.o.min(c.c), "Thân"), (c.o.min(c.c), c.l, "Bóng dưới")];
            for (a, b, name) in parts {
                let (ya, yb) = (y(a), y(b));
                if yb - ya < 3.0 {
                    continue;
                }
                svg += &format!(r#"<path class="nen-chi" d="M{:.1} {ya:.1} H{lx:.1} V{yb:.1} H{:.1}"/>"#, lx + 5.0, lx + 5.0);
                svg += &format!(r#"<text class="nen-chu" x="{:.1}" y="{:.1}" text-anchor="end">{name}</text>"#, lx - 4.0, (ya + yb) / 2.0 + 4.0);
            }
        }
    }

    let title = f.title.clone().unwrap_or_else(|| "Hình nến minh hoạ".into());
    let desc = describe(&f);
    Ok(format!(
        r#"<figure class="hinh-nen"><svg viewBox="0 0 {width:.0} {height:.0}" width="{width:.0}" height="{height:.0}" role="img" aria-label="{}"><title>{}</title>{svg}</svg><figcaption>{}</figcaption></figure>"#,
        escape_html(&format!("{title}. {desc}")),
        escape_html(&title),
        escape_html(&title)
    ))
}

/// Mô tả ngắn bằng chữ cho trình đọc màn hình.
fn describe(f: &Figure) -> String {
    let n = f.slots.iter().flatten().count();
    let marked: Vec<&str> = f.slots.iter().flatten().filter_map(|c| c.label.as_deref()).collect();
    let mut s = format!("{n} cây nến");
    if !marked.is_empty() {
        s += &format!(", đánh dấu: {}", marked.join(", "));
    }
    for (p, t) in &f.lines {
        s += &format!("; đường {t} ở {}", fmt_price(*p));
    }
    s
}

/// Khung báo lỗi hiện thay cho hình khi khối ```nen viết sai.
pub fn error_box(msg: &str) -> String {
    format!(r#"<p class="callout callout-bad">Hình nến bị lỗi: {}</p>"#, escape_html(msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ve_nen_va_bao_loi() {
        let svg = render("ten: Búa\n110 111 103 104\n104 105 98 99\n96 97.5 90 97 | Búa\nngang 90 Hỗ trợ").unwrap();
        assert!(svg.starts_with("<figure class=\"hinh-nen\"><svg"));
        assert_eq!(svg.matches("nen-than").count(), 3);
        assert!(svg.contains("nen-giam") && svg.contains("nen-tang"));
        assert!(svg.contains(">Búa</text>") && svg.contains("Hỗ trợ <tspan class=\"nen-gia\">90</tspan>"));
        assert!(render("100 90 95 98").unwrap_err().contains("giá cao nhất"));
        assert!(render("100 108 97").unwrap_err().contains("4 số"));
        assert!(render("ten: trống").is_err());
        let z = render("55 58 52 53
53 57 50 56
56 70 55 69
vung 50 60 Gom hàng @1-2").unwrap();
        assert!(z.contains("text-anchor=\"middle\">Gom hàng</text>"));
        assert!(render("55 58 52 53
vung 50 60 X @3-1").unwrap_err().contains("@3-8"));
        let g = render("giai-phau\n100 108 97 105").unwrap();
        assert!(g.contains("Giá đóng cửa <tspan class=\"nen-gia\">105</tspan>") && g.contains(">Bóng dưới</text>"));
    }

    #[test]
    fn gia_kieu_viet_nam() {
        assert_eq!(fmt_price(57300.0), "57.300");
        assert_eq!(fmt_price(1250000.0), "1.250.000");
        assert_eq!(fmt_price(97.5), "97,5");
        assert_eq!(fmt_price(105.0), "105");
        assert_eq!(fmt_price(2000.0), "2.000");
    }
}
