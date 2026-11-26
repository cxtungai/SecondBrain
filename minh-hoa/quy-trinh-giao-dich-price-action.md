# Hình minh hoạ cho: quy-trinh-giao-dich-price-action

Chỉ dùng trên trang web; không thuộc kho tri thức. Mỗi mục "## …" trùng tên một mục trong bài, hình được chèn vào cuối mục đó. Cách viết khối nến: xem `src/nen.rs`.

## Bước 3 — Chờ setup hợp bối cảnh

```nen
ten: Phá vỡ – điều chỉnh: thoát khỏi vùng đi ngang, điều chỉnh vài thanh, rồi vào lệnh theo hướng phá vỡ
99 101.5 98.6 101
101 101.8 99 99.4
99.4 101.7 98.8 101.3
101.3 101.9 99.2 99.6
99.6 104.8 99.5 104.5 | Phá vỡ
104.5 104.9 103 103.4
103.4 103.8 102.2 102.8
102.8 105 102.6 104.8 | Vào lệnh
104.8 107.5 104.6 107.2
vung 98.6 102 Vùng đi ngang @1-4
```

## Ví dụ áp dụng

```nen
ten: Ví dụ trong bài: điều chỉnh về 56.000 đồng, thanh tín hiệu đóng 57.000 — mua dừng 57.300, dừng lỗ 55.700, mục tiêu 2R và 3R
52000 53600 51800 53400
53400 55200 53200 55000
55000 57400 54800 57200
57200 59400 57000 59200
59200 60000 58300 58600
58600 58900 57500 57700
57700 57900 56700 56900
56900 57100 55900 56100
56100 57200 55800 57000 | Thanh tín hiệu
ngang 62100 Mục tiêu 3R
ngang 60500 Mục tiêu 2R
ngang 57300 Mua dừng
ngang 55700 Dừng lỗ
```
