# Hình minh hoạ cho: price-action

Chỉ dùng trên trang web; không thuộc kho tri thức. Mỗi mục "## …" trùng tên một mục trong bài, hình được chèn vào cuối mục đó. Cách viết khối nến: xem `src/nen.rs`.

## Bốn thứ người giao dịch price action đọc

```nen
ten: Đọc theo thứ tự — bối cảnh (xu hướng tăng) → vùng giá (đỉnh cũ) → tín hiệu (thanh tăng đóng gần đỉnh) → kế hoạch lệnh
100 103 99.5 102.6
102.6 106 102.2 105.6
105.6 109 105.2 108.6
108.6 108.9 106.4 106.8
106.8 107.2 104.8 105.2
105.2 107.6 104.4 107.4 | Tín hiệu
vung 104.6 106 Vùng giá
ngang 107.7 Vào lệnh
ngang 104.2 Dừng lỗ
```
