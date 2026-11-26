# Hình minh hoạ cho: mau-hinh-nen

Chỉ dùng trên trang web; không thuộc kho tri thức. Mỗi mục "## …" trùng tên một mục trong bài, hình được chèn vào cuối mục đó. Cách viết khối nến: xem `src/nen.rs`.

## Nhóm 1 — nến bóng dài (một cây): búa, người treo cổ, sao băng, búa ngược

```nen
ten: Búa — bóng dưới dài, sau đà giảm
112 113 106 107
107 108 102 103
103 104 98 99
97.5 99 91 98.5 | Búa
```

```nen
ten: Người treo cổ — cùng hình dạng, nhưng sau đà tăng
88 93 87 92
92 97 91 96
96 101 95 100
102 102.3 95.5 101.6 | Người treo cổ
```

```nen
ten: Sao băng — bóng trên dài, sau đà tăng
88 93 87 92
92 97 91 96
96 101 95 100
101 108 100.6 101.5 | Sao băng
```

```nen
ten: Búa ngược — cùng hình dạng, nhưng sau đà giảm
112 113 106 107
107 108 102 103
103 104 98 99
98 104.5 97.6 98.6 | Búa ngược
```

## Nhóm 2 — nhấn chìm (hai cây)

```nen
ten: Nhấn chìm tăng — thân nến tăng bao trùm thân nến giảm trước
112 113 106 107
107 108 102 103
102.5 103 100 100.8
100.2 106 99.5 105.5 | Nhấn chìm tăng
```

```nen
ten: Nhấn chìm giảm — đối xứng, sau đà tăng
88 93 87 92
92 97 91 96
97.5 100 97 99.2
99.8 100.5 94 94.5 | Nhấn chìm giảm
```

```nen
ten: Xuyên đường — mở dưới giá đóng cửa trước, đóng trên điểm giữa thân
112 113 106 107
107 108 101 102
100.5 106 100 105.2 | Xuyên đường
ngang 104.5 Điểm giữa thân
```

```nen
ten: Mây đen che phủ — mở trên giá đóng cửa trước, đóng dưới điểm giữa thân
88 93 87 92
93 99 92.5 98
99.5 100 94.5 95 | Mây đen
ngang 95.5 Điểm giữa thân
```

## Nhóm 3 — harami (hai cây, gần với "inside bar")

```nen
ten: Harami — thân nhỏ nằm trọn trong thân lớn trước đó
112 113 106 107
108 108.5 99 99.5
102 103.5 101 103 | Harami
```

```nen
ten: Inside bar — cả biên độ nằm trong biên độ thanh trước
100 110 95 105
103 107 99 101 | Inside bar
```

## Nhóm 4 — sao mai, sao hôm (ba cây)

```nen
ten: Sao mai — nến giảm dài, nến nhỏ mở thấp hơn, rồi nến tăng dài
112 113 107 108
108 108.5 100 100.5
99 100 97.5 99.4 | Sao mai
100 107.5 99.5 107
```

```nen
ten: Sao hôm — đối xứng, sau đà tăng
88 93 87 92
92 99.5 91.5 99
100.5 102 100 100.8 | Sao hôm
100 100.5 93 93.5
```

## Nhóm 5 — doji

```nen
ten: Doji — mở bằng đóng
100 106 94 100
```

```nen
ten: Doji chuồn chuồn
100 100 92 100
```

```nen
ten: Doji bia mộ
100 108 100 100
```

## Cách Al Brooks đánh giá một thanh tín hiệu

```nen
ten: Thanh đảo chiều tăng tốt — thân tăng rõ, đóng cửa gần đỉnh
112 113 107 108
108 109 103 104
103 104 99 100
100 105.5 98.5 105.2 | Thanh tốt
```

```nen
ten: Thanh yếu — thân nhỏ, bóng dài ở phía đối diện
112 113 107 108
108 109 103 104
103 104 99 100
100.5 103 99 100.8 | Thanh yếu
```

## Ví dụ áp dụng

```nen
ten: Nến búa ở hỗ trợ 50.000 đồng, phiên sau xác nhận
56000 56300 54600 54800
54800 55000 53200 53400
53400 53600 51800 52000
52000 52300 50400 50600
49800 50300 48500 50200 | Búa
50200 52500 50100 52400 | Xác nhận
ngang 50000 Hỗ trợ cũ
```
