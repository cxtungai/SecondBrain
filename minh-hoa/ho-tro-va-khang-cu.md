# Hình minh hoạ cho: ho-tro-va-khang-cu

Chỉ dùng trên trang web; không thuộc kho tri thức. Mỗi mục "## …" trùng tên một mục trong bài, hình được chèn vào cuối mục đó. Cách viết khối nến: xem `src/nen.rs`.

## Trong tài liệu gốc của các phương pháp

```nen
ten: "Điểm kháng cự" của Wyckoff (1910): bán khống ở 130, giá về 128, hồi lên 129 rồi quay xuống — dừng lỗ đặt ngay trên 129
130.8 131.2 129.8 130.1 | Bán khống
130.1 130.3 127.8 128.1
128.1 129 127.9 128.8
128.8 128.95 126.8 127.1
ngang 129 Điểm kháng cự
```

```nen
ten: Wyckoff (1922): lên từ 100 tới 150, bị đánh xuống 110, rồi chống đỡ được trước áp lực bán
100 112 99 111
111 124 110 123
123 137 122 136
136 150 135 148
148 149 136 138
138 139 124 126
126 127 112 114
114 117 109.8 116
116 118 110.2 113
113 116 110 115.5 | Chống đỡ
ngang 110 Hỗ trợ
```

## Ví dụ

```nen
ten: Đi ngang 40.000–46.000, phá lên 47.500, quay lại kiểm tra 46.000 rồi bật lên — kháng cự cũ thành hỗ trợ mới
41000 43500 40300 43000
43000 45800 42800 45200
45200 45600 43000 43400
43400 43800 40400 41000
41000 43900 40200 43600
43600 45900 43400 45500
45500 45700 42900 43200
43200 45800 42800 45600
45600 47800 45400 47500 | Phá vỡ
47400 47900 46800 47500
47500 47600 46500 46700
46700 46900 46200 46600
46600 48200 46400 48000 | Kiểm tra lại
ngang 46000 Kháng cự cũ
ngang 40000 Hỗ trợ
```
