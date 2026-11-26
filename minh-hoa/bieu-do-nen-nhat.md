# Hình minh hoạ cho: bieu-do-nen-nhat

Chỉ dùng trên trang web; không thuộc kho tri thức. Mỗi mục "## …" trùng tên một mục trong bài, hình được chèn vào cuối mục đó. Cách viết khối nến: xem `src/nen.rs`.

## Cấu tạo một cây nến

```nen
ten: Nến tăng — đóng cửa cao hơn mở cửa
giai-phau
100 108 97 105
```

```nen
ten: Nến giảm — đóng cửa thấp hơn mở cửa
giai-phau
105 108 97 100
```

## Đọc thân nến: ai thắng, thắng mạnh hay yếu

```nen
ten: Thân tăng dài — bên mua áp đảo
100 112 99 111
```

```nen
ten: Thân giảm dài — bên bán áp đảo
111 112 99 100
```

```nen
ten: Marubozu tăng — không có bóng
100 112 100 112
```

```nen
ten: Thân ngắn — hai bên giằng co
105 109 101 106
```

## Đọc bóng nến: giá đã bị từ chối ở đâu

```nen
ten: Bóng trên dài — bị bán xuống từ đỉnh
100 110 99 102
```

```nen
ten: Bóng dưới dài — được mua lên từ đáy
100 102 90 101
```

```nen
ten: Con quay — thân nhỏ, hai bóng dài
100 106 94 101
```

```nen
ten: Doji — giá mở bằng giá đóng
100 106 94 100
```

```nen
ten: Doji chuồn chuồn — bóng dưới dài
100 100 92 100
```

```nen
ten: Doji bia mộ — bóng trên dài
100 108 100 100
```

## Đọc nến trong bối cảnh

```nen
ten: Bóng dưới dài sau đà giảm, chạm hỗ trợ — có ý nghĩa
120 121 114 115
115 116 110 111
111 112 106 107
107 109 100 108 | Bóng dưới dài
ngang 100 Hỗ trợ
```

```nen
ten: Cùng hình dạng nhưng giữa vùng đi ngang chồng chéo — chỉ là đi ngang
108 111 104 106
106 110 103 109
109 111 104 105
105 110 101 108 | Bóng dưới dài
```
