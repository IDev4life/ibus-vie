# INPUT_METHODS — Quy tắc gõ Telex / VNI / VIQR

Tài liệu này mô tả các kiểu gõ mà `ibus-vie` hỗ trợ. Đây là phần _thuần thuật toán_ — không liên quan IBus.

Các bảng quy tắc dưới đây dựa trên quy ước phổ biến từ Unikey và các bộ gõ tiếng Việt khác. Đã được xác minh qua snapshot tests trong `tests/snapshot/`.

---

## 1. Khung chung

Mọi kiểu gõ đều dùng cùng một mô hình:

1. **Phím nhấn** vào → cập nhật **preedit buffer** (chuỗi ký tự thô đang gõ).
2. **Bộ luật biến đổi** áp lên buffer, tạo ra **chuỗi tiếng Việt** đang dự đoán.
3. Hiển thị chuỗi tiếng Việt như preedit (gạch chân).
4. Khi gặp **điểm commit** (space, ký tự không hợp lệ, enter, ...) → commit chuỗi.

Cách tiếp cận "buffer + rewrite" này tương tự Unikey và ibus-bogo — đơn giản hơn parse syllable đầy đủ và đủ chính xác cho dùng hàng ngày.

---

## 2. Telex

Telex là kiểu gõ phổ biến nhất ở Việt Nam.

### 2.1. Nguyên âm có dấu phụ

| Gõ   | Thành | Ghi chú |
| ---- | ----- | ------- |
| `aa` | â     |         |
| `aw` | ă     |         |
| `ee` | ê     |         |
| `oo` | ô     |         |
| `ow` | ơ     |         |
| `uw` | ư     |         |
| `dd` | đ     |         |

### 2.2. Dấu thanh

Đặt sau âm tiết. Áp dấu lên nguyên âm theo quy tắc chính tả tiếng Việt.

| Phím | Dấu             |
| ---- | --------------- |
| `s`  | sắc (´)         |
| `f`  | huyền (`)       |
| `r`  | hỏi (ˀ)         |
| `x`  | ngã (˜)         |
| `j`  | nặng (.)        |
| `z`  | xoá dấu hiện có |

### 2.3. Ví dụ

| Gõ          | Kết quả |
| ----------- | ------- |
| `vieetj`    | việt    |
| `tieengs`   | tiếng   |
| `Vieetj`    | Việt    |
| `ddaau`     | đâu     |
| `nguwowif`  | người   |
| `hoaf`      | hoà     |
| `chaof`     | chào    |
| `truwowngf` | trường  |

Vị trí đặt dấu mặc định: **kiểu mới** ("hòa"). Có thể đổi sang kiểu cũ ("hoà") qua config `tone_style = "old"` trong `~/.config/ibus-vie/config.toml`.

### 2.4. Quy tắc quan trọng

- **Hoàn tác bằng z**: gõ `z` sau một nguyên âm có dấu để xoá dấu/thanh đó. Ví dụ `awz` → `aw` → `a`.
- **Lặp ký tự để "xin lỗi"**: nếu người dùng thực sự muốn gõ "aa" tiếng Anh, một số bộ gõ cho phép gõ `aaa` để huỷ biến đổi và giữ nguyên `aa`. [Chưa xác minh] ibus-vie có thể adopt quy ước này hoặc dùng cách khác. Quyết định khi prototype.
- **Backspace trong preedit**: huỷ thao tác cuối, không xoá ký tự đã hiển thị thành chữ Việt.

### 2.5. Test case (từ `tests/snapshot/telex.txt`)

```
Input            Expected
─────────────    ─────────────
vieetj           việt
tieengs          tiếng
ddi              đi
chaof            chào
ddaau            đâu
nguwowif         người
xin              xin
hoaf             hoà
khoor            khỏ
thuees           thuế
truwowngf        trường
Vieetj           Việt
```

Gõ `truwowngf` (không phải "trườngf") vì phải gõ ư và ơ bằng `uw` và `ow` trước khi đặt dấu.

---

## 3. VNI

Kiểu gõ dùng số. Phổ biến ở miền Nam và cộng đồng người Việt hải ngoại.

### 3.1. Nguyên âm có dấu phụ

| Gõ   | Thành |
| ---- | ----- |
| `a6` | â     |
| `a8` | ă     |
| `e6` | ê     |
| `o6` | ô     |
| `o7` | ơ     |
| `u7` | ư     |
| `d9` | đ     |

### 3.2. Dấu thanh

| Phím | Dấu     |
| ---- | ------- |
| `1`  | sắc     |
| `2`  | huyền   |
| `3`  | hỏi     |
| `4`  | ngã     |
| `5`  | nặng    |
| `0`  | xoá dấu |

### 3.3. Ví dụ (từ `tests/snapshot/vni.txt`)

| Gõ          | Kết quả |
| ----------- | ------- |
| `vie6t5`    | việt    |
| `d9i`       | đi      |
| `d9a6u`     | đâu     |
| `ba2n`      | bàn     |
| `to6i1`     | tối     |
| `sa1ng`     | sáng    |
| `chie6u2`   | chiều   |
| `ngu7o7i2`  | người   |
| `thu7o7ng2` | thường  |
| `la8m1`     | lắm     |
| `tu75`      | tự      |

Thứ tự: gõ dấu phụ ngay sau nguyên âm (vd `e6` → ê), dấu thanh đặt cuối âm tiết (vd `5` → nặng).

---

## 4. VIQR

Vietnamese Quoted-Readable — dùng ASCII punctuation. Ít phổ biến hơn ở người dùng cuối, nhưng vẫn được dùng trong môi trường text-only và email cổ.

### 4.1. Nguyên âm có dấu phụ

| Gõ   | Thành |
| ---- | ----- |
| `a^` | â     |
| `a(` | ă     |
| `e^` | ê     |
| `o^` | ô     |
| `o+` | ơ     |
| `u+` | ư     |
| `dd` | đ     |

### 4.2. Dấu thanh

| Phím    | Dấu   |
| ------- | ----- |
| `'`     | sắc   |
| `` ` `` | huyền |
| `?`     | hỏi   |
| `~`     | ngã   |
| `.`     | nặng  |

### 4.3. Ví dụ (từ `tests/snapshot/viqr.txt`)

| Gõ         | Kết quả |
| ---------- | ------- |
| `Vie^.t`   | Việt    |
| `dda^u`    | đâu     |
| `ba`n`     | bàn     |
| `to^'i`    | tối     |
| `sa'ng`    | sáng    |
| `chie^`u`  | chiều   |
| `ngu+o+`i` | người   |
| `la(m'`    | lắm     |
| `tu+.`     | tự      |

VIQR dùng các ký tự ASCII punctuation (`'`, `` ` ``, `?`, `~`, `.`) nên engine cần cơ chế phân biệt khi nào ký tự là dấu thanh vs dấu câu.

---

## 5. Quy tắc đặt dấu thanh (chung cho cả 3 kiểu)

Dấu thanh đặt ở nguyên âm chính của vần. Quy tắc phổ biến:

1. Vần có nguyên âm đôi/ba: dấu đặt trên nguyên âm "chính" (nguyên âm thứ hai trong các tổ hợp như "oa", "uy", "uê").
2. Có hai trường phái:
   - **Kiểu cũ:** dấu trên nguyên âm đầu (vd: "hoà", "thuý").
   - **Kiểu mới:** dấu trên nguyên âm sau (vd: "hòa", "thúy").

**Mặc định:** kiểu mới. Đổi sang kiểu cũ qua `tone_style = "old"` trong config.

---

## 6. Phương pháp test

Engine FSM được tách ra khỏi IBus glue (xem `ARCHITECTURE.md`), test chạy hoàn toàn offline:

```rust
#[test]
fn telex_basic() {
    let mut e = TelexEngine::new();
    assert_eq!(e.feed_str("vieetj"), "việt");
    assert_eq!(e.feed_str("ddi"),    "đi");
    assert_eq!(e.feed_str("tieengs"),"tiếng");
}
```

Test case lớn lưu dưới dạng snapshot file (`tests/snapshot/<method>.txt`) — người không biết Rust cũng có thể đóng góp bằng cách thêm dòng:

```
# tests/snapshot/telex.txt
vieetj	việt
tieengs	tiếng
ddi	đi
```

Bộ test hiện tại và cần mở rộng:

- Từ thông dụng cho mỗi kiểu gõ.
- Trường hợp khó: nguyên âm đôi, ba; dấu thanh trên ư/ơ; "qu", "gi".
- Test "huỷ thao tác": `z`, backspace.
- Viết hoa (Telex: `Vieetj` → `Việt`).
