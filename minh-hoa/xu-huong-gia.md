# Hình minh hoạ cho: xu-huong-gia

Chỉ dùng trên trang web; không thuộc kho tri thức. Mỗi mục "## …" trùng tên một mục trong bài, hình được chèn vào cuối mục đó. Cách viết khối nến: xem `src/nen.rs`.

## "Đường đi ngang": tích luỹ hay phân phối

```nen
ten: Giá đi ngang trong biên độ hẹp rồi vượt lên trên "đường"
99 101.5 98.5 101
101 102 99 99.5
99.5 101.8 98.8 101.2
101.2 101.9 98.6 99
99 101.6 98.7 101.4
101.4 101.8 99.2 99.8
99.8 105.5 99.6 105.2 | Vượt lên
105.2 107.2 104.6 106.8
vung 98.5 102 Đường đi ngang @1-6
```

## Thuật ngữ của Brooks

```nen
ten: High 1 và High 2 trong một nhịp điều chỉnh của xu hướng tăng
100 103 99.5 102.5
102.5 106 102 105.5
105.5 109 105 108.5
108.5 108.8 106.5 107
107 107.5 105 105.5
105.5 107.8 105.2 107.4 | H1
107.4 107.6 104.8 105.2
105.2 105.6 104 104.6
104.6 107.9 104.4 107.6 | H2
107.6 111 107.4 110.6
```

## Ví dụ

```nen
ten: Đáy 100 → đỉnh 110 → điều chỉnh về 106 (lấy lại 4/10) → vượt lên 115
101 103 100 102.5
102.5 105.5 102 105
105 108 104.5 107.5
107.5 110 107 108.2
108.2 108.5 106.5 107
107 107.5 106 106.8 | Đáy điều chỉnh
106.8 109.5 106.5 109.2
109.2 112.5 109 112
112 115 111.5 114.5 | Đỉnh mới
ngang 110 Đỉnh trước
```
