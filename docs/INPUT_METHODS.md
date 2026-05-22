# INPUT_METHODS — Quy tắc gõ Telex / VNI / VIQR

Tài liệu này mô tả các kiểu gõ mà `ibus-vie` hỗ trợ. Đây là phần *thuần thuật toán* — không liên quan IBus.

[Chưa xác minh] Các bảng quy tắc dưới đây là quy ước phổ biến được thừa hưởng từ Unikey và các bộ gõ tiếng Việt khác. Một vài biến thể có thể tồn tại; khi prototype phải đối chiếu với tài liệu Unikey gốc hoặc test suite của ibus-bogo.

---

## 1. Khung chung

Mọi kiểu gõ đều dùng cùng một mô hình:

1. **Phím nhấn** vào → cập nhật **preedit buffer** (chuỗi ký tự thô đang gõ).
2. **Bộ luật biến đổi** áp lên buffer, tạo ra **chuỗi tiếng Việt** đang dự đoán.
3. Hiển thị chuỗi tiếng Việt như preedit (gạch chân).
4. Khi gặp **điểm commit** (space, ký tự không hợp lệ, enter, ...) → commit chuỗi.

[Suy luận] Cách tiếp cận "buffer + rewrite" này là cách Unikey và ibus-bogo dùng. Nó đơn giản hơn parse syllable đầy đủ và đủ chính xác cho dùng hàng ngày.

---

## 2. Telex

Telex là kiểu gõ phổ biến nhất ở Việt Nam.

### 2.1. Nguyên âm có dấu phụ

| Gõ | Thành | Ghi chú |
|----|-------|---------|
| `aa` | â | |
| `aw` | ă | |
| `ee` | ê | |
| `oo` | ô | |
| `ow` | ơ | |
| `uw` | ư | |
| `dd` | đ | |

### 2.2. Dấu thanh

Đặt sau âm tiết. Áp dấu lên nguyên âm theo quy tắc chính tả tiếng Việt.

| Phím | Dấu |
|------|-----|
| `s` | sắc (´) |
| `f` | huyền (`) |
| `r` | hỏi (ˀ) |
| `x` | ngã (˜) |
| `j` | nặng (.) |
| `z` | xoá dấu hiện có |

### 2.3. Ví dụ

| Gõ | Kết quả |
|----|---------|
| `vieetj` | việt |
| `tieengs` | tiếng |
| `Vieetj Nam` | Việt Nam |
| `ddaau` | đâu |
| `hoaf` | hoà *(hoặc "hòa" tuỳ quy ước đặt dấu)* |

[Chưa xác minh] Vị trí đặt dấu giữa "hoà" / "hòa" phụ thuộc lựa chọn quy ước. Mặc định đề xuất dùng quy ước **mới** ("hòa") vì phổ biến hơn trên web hiện đại. Cần cấu hình được.

### 2.4. Quy tắc quan trọng

- **Hoàn tác bằng z**: gõ `z` sau một nguyên âm có dấu để xoá dấu/thanh đó. Ví dụ `awz` → `aw` → `a`.
- **Lặp ký tự để "xin lỗi"**: nếu người dùng thực sự muốn gõ "aa" tiếng Anh, một số bộ gõ cho phép gõ `aaa` để huỷ biến đổi và giữ nguyên `aa`. [Chưa xác minh] ibus-vie có thể adopt quy ước này hoặc dùng cách khác. Quyết định khi prototype.
- **Backspace trong preedit**: huỷ thao tác cuối, không xoá ký tự đã hiển thị thành chữ Việt.

### 2.5. Test case mẫu

```
Input            Expected
─────────────    ─────────────
chaof            chào
trườngf          *(ambiguous — phải test)*
ddoongf          đồng
khoor            khở
nguwowif         người
Vieetj Nam       Việt Nam
ddi laif         đi lại
```

[Chưa xác minh] Test case "trườngf" cần được kiểm tra với prototype — có thể là "trườngf" → "trường" (đã có sẵn dấu) hoặc cần gõ "truwowngf".

---

## 3. VNI

Kiểu gõ dùng số. Phổ biến ở miền Nam và cộng đồng người Việt hải ngoại.

### 3.1. Nguyên âm có dấu phụ

| Gõ | Thành |
|----|-------|
| `a6` | â |
| `a8` | ă |
| `e6` | ê |
| `o6` | ô |
| `o7` | ơ |
| `u7` | ư |
| `d9` | đ |

### 3.2. Dấu thanh

| Phím | Dấu |
|------|-----|
| `1` | sắc |
| `2` | huyền |
| `3` | hỏi |
| `4` | ngã |
| `5` | nặng |
| `0` | xoá dấu |

### 3.3. Ví dụ

| Gõ | Kết quả |
|----|---------|
| `vie65t5` | việt *(viết sai — đúng phải là `vie65t` rồi `5`)* |
| `Vie65t Nam` | Việt Nam *(thứ tự cụ thể cần kiểm)* |
| `d9a6u` | đâu |

[Chưa xác minh] Thứ tự gõ dấu thanh trong VNI có quy ước cụ thể (ngay sau nguyên âm có dấu, hay cuối âm tiết). Phải đối chiếu Unikey docs. Bảng trên là phác thảo; tests sẽ làm rõ.

---

## 4. VIQR

Vietnamese Quoted-Readable — dùng ASCII punctuation. Ít phổ biến hơn ở người dùng cuối, nhưng vẫn được dùng trong môi trường text-only và email cổ.

### 4.1. Nguyên âm có dấu phụ

| Gõ | Thành |
|----|-------|
| `a^` | â |
| `a(` | ă |
| `e^` | ê |
| `o^` | ô |
| `o+` | ơ |
| `u+` | ư |
| `dd` | đ |

### 4.2. Dấu thanh

| Phím | Dấu |
|------|-----|
| `'` | sắc |
| `` ` `` | huyền |
| `?` | hỏi |
| `~` | ngã |
| `.` | nặng |

### 4.3. Ví dụ

| Gõ | Kết quả |
|----|---------|
| `Vie^.t Nam` | Việt Nam |
| `dda^u` | đâu |

[Suy luận] VIQR có vấn đề thực tế là các ký tự `'`, `` ` ``, `?`, `~`, `.` rất hay dùng trong văn bản thông thường (ví dụ dấu chấm câu). Engine phải có cơ chế escape tốt — VIQR chuẩn dùng `\` để escape. [Chưa xác minh] Cần đối chiếu RFC 1456 (đặc tả VIQR) khi implement.

---

## 5. Quy tắc đặt dấu thanh (chung cho cả 3 kiểu)

Dấu thanh đặt ở nguyên âm chính của vần. Quy tắc phổ biến:

1. Vần có nguyên âm đôi/ba: dấu đặt trên nguyên âm "chính" (nguyên âm thứ hai trong các tổ hợp như "oa", "uy", "uê").
2. Có hai trường phái:
   - **Kiểu cũ:** dấu trên nguyên âm đầu (vd: "hoà", "thuý").
   - **Kiểu mới:** dấu trên nguyên âm sau (vd: "hòa", "thúy").

[Chưa xác minh] Quyết định mặc định kiểu nào cần dựa trên khảo sát thực tế. Đề xuất: **kiểu mới làm mặc định**, có config để đổi sang kiểu cũ.

---

## 6. Phương pháp test

[Suy luận] Vì engine FSM được tách ra khỏi IBus glue (xem `ARCHITECTURE.md`), test có thể chạy hoàn toàn offline:

```rust
// Pseudocode — sẽ trở thành test thật trong crates/ibus-vie-im/tests/
#[test]
fn telex_basic() {
    let mut e = TelexEngine::new();
    assert_eq!(e.feed_str("vieetj"), "việt");
    assert_eq!(e.feed_str("ddi"),    "đi");
    assert_eq!(e.feed_str("tieengs"),"tiếng");
}
```

Ngoài ra, các test case lớn được lưu dưới dạng snapshot file (xem `SOURCE_LAYOUT.md` §6) để người không biết Rust cũng có thể đóng góp:

```
# tests/snapshot/telex.txt
vieetj    việt
tieengs   tiếng
ddi       đi
Vieetj Nam    Việt Nam
```

Cần xây dựng bộ test gồm ít nhất:
- 100 từ thông dụng cho mỗi kiểu gõ.
- Các trường hợp khó: nguyên âm đôi, ba; dấu thanh trên ư/ơ; "qu", "gi".
- Test "huỷ thao tác": `z`, backspace, lặp phụ âm.