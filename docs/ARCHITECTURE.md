# ARCHITECTURE — Kiến trúc kỹ thuật

Tài liệu này mô tả cách `ibus-vie` được tổ chức bên trong. Cần đọc `SPEC.md` trước.

---

## 1. Tổng quan

```
┌─────────────────────────────────────────────────────────────┐
│  Ứng dụng (Firefox, GNOME Text Editor, LibreOffice, ...)    │
└─────────────────────────────────────────────────────────────┘
                          ▲
                          │  Wayland text-input-v3 protocol
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  Compositor: Mutter (GNOME) hoặc KWin (KDE)                 │
└─────────────────────────────────────────────────────────────┘
                          ▲
                          │  zwp_input_method_v2 hoặc tương đương
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  ibus-daemon (đã có sẵn trong GNOME, hoặc cài thêm trên KDE)│
└─────────────────────────────────────────────────────────────┘
                          ▲
                          │  IBus IPC (DBus)
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  ibus-vie engine  ←  ĐÂY LÀ PHẦN REPO NÀY VIẾT                │
│  ┌───────────────┐  ┌──────────────┐  ┌─────────────────┐   │
│  │ IBus Engine   │  │ Input Method │  │ Vietnamese      │   │
│  │ glue (DBus)   │──│ FSM          │──│ syllable rules  │   │
│  └───────────────┘  └──────────────┘  └─────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

[Chưa xác minh] Sơ đồ trên là cách hiểu chuẩn về stack input method trên Wayland qua IBus. Một số chi tiết (ví dụ phiên bản giao thức Wayland chính xác mà Mutter dùng để nói chuyện với IBus) cần được tra cứu lại trước khi viết code.

---

## 2. Vì sao là IBus engine, không phải Wayland input method trực tiếp

Có hai con đường khả dĩ:

### Con đường A — Viết IBus engine _(ibus-vie chọn cái này)_

- Engine nói chuyện với `ibus-daemon` qua DBus.
- `ibus-daemon` nói chuyện với compositor.
- **Lợi:** GNOME Settings → Input Sources tự động nhận. Người dùng "chỉ cần add input source".
- **Bất lợi:** Phụ thuộc IBus.

### Con đường B — Viết Wayland input method client trực tiếp

- Engine kết nối thẳng tới compositor qua `zwp_input_method_v2`.
- **Lợi:** Không cần IBus.
- **Bất lợi:** Settings không có chỗ liệt kê. Người dùng phải tự khởi chạy, tự quản lý — phá yêu cầu G1 của SPEC.

[Suy luận] Vì yêu cầu cốt lõi của repo này là "chỉ cần add trong Settings", **con đường A là bắt buộc**. Con đường B đáng cân nhắc cho phiên bản tương lai nếu cần thoát hoàn toàn khỏi IBus, nhưng nó kéo theo việc tự viết phần "đăng ký vào Settings" — rất phức tạp và phụ thuộc desktop environment.

---

## 3. Các thành phần

### 3.1. Component descriptor (XML)

File: `vie.xml`, cài tại `/usr/share/ibus/component/vie.xml`.

[Suy luận] Nội dung kỳ vọng (chính xác cú pháp cần đối chiếu IBus docs khi cài thật):

```xml
<?xml version="1.0" encoding="utf-8"?>
<component>
  <name>org.freedesktop.IBus.Vie</name>
  <description>Vietnamese input method (ibus-vie)</description>
  <exec>/usr/libexec/ibus-engine-vie --ibus</exec>
  <version>0.1.0</version>
  <author>...</author>
  <license>...</license>
  <homepage>...</homepage>
  <textdomain>ibus-vie</textdomain>

  <engines>
    <engine>
      <name>vie-telex</name>
      <language>vi</language>
      <license>...</license>
      <author>...</author>
      <layout>us</layout>
      <longname>Vietnamese (ibus-vie — Telex)</longname>
      <description>Vietnamese Telex via ibus-vie</description>
      <symbol>VI</symbol>
    </engine>
    <engine>
      <name>vie-vni</name>
      <language>vi</language>
      <layout>us</layout>
      <longname>Vietnamese (ibus-vie — VNI)</longname>
      <symbol>VI</symbol>
    </engine>
    <engine>
      <name>vie-viqr</name>
      <language>vi</language>
      <layout>us</layout>
      <longname>Vietnamese (ibus-vie — VIQR)</longname>
      <symbol>VI</symbol>
    </engine>
  </engines>
</component>
```

[Chưa xác minh] Cú pháp XML cụ thể của IBus có thể có thêm/bớt trường so với ví dụ trên. Phải đối chiếu với tài liệu IBus tại thời điểm implement.

### 3.2. Engine executable

Binary chạy khi IBus kích hoạt engine. Các quyết định:

- **Ngôn ngữ:** **Rust** (edition 2021). Không phụ thuộc runtime ngoài (`glibc` là đủ), binary nhỏ, hiệu năng tốt, an toàn bộ nhớ — phù hợp với một process long-running do `ibus-daemon` spawn/kill.
- **Vị trí cài:** `/usr/libexec/ibus-engine-vie`.
- **Vòng đời:** `ibus-daemon` tự spawn khi engine được activate, terminate khi không cần.

[Suy luận] Rust được chọn thay vì Python (tránh phụ thuộc Python runtime + GI bindings cho mỗi user) và thay vì C (tránh nợ kỹ thuật về quản lý bộ nhớ). Đánh đổi: thời gian đến prototype đầu tiên dài hơn Python một chút, nhưng từ Phase 1 trở đi sẽ thuận hơn.

**Giao tiếp với IBus:**

[Chưa xác minh] Phương án ưu tiên: dùng crate `zbus` (Rust thuần) để nói chuyện trực tiếp với `ibus-daemon` qua DBus. IBus expose interface qua DBus, nên về lý thuyết không cần libibus C library. Phương án dự phòng nếu `zbus` thiếu API: dùng `gtk-rs` + binding FFI vào libibus.

Crate phụ thuộc chính (dự kiến):

| Crate                            | Vai trò                                           |
| -------------------------------- | ------------------------------------------------- |
| `zbus`                           | DBus client async, Rust thuần                     |
| `tokio`                          | Async runtime (single-thread flavor)              |
| `tracing` + `tracing-subscriber` | Structured logging cho debug                      |
| `serde` + `toml`                 | Đọc file config user                              |
| `clap`                           | CLI parsing (cho `--ibus` flag và `ibus-vie-cli`) |
| `insta`                          | Snapshot testing                                  |
| `criterion` (dev)                | Benchmark FSM                                     |

Chi tiết phân bổ source xem **`SOURCE_LAYOUT.md`**.

### 3.3. Input Method FSM (Finite State Machine)

Phần xử lý phím nhấn → preedit → commit. Tách thành module thuần (không phụ thuộc IBus) để dễ test.

Chi tiết quy tắc xem `INPUT_METHODS.md`.

**API nội bộ đề xuất:**

```
struct Engine {
    fn key_press(key: Key) -> Action
}

enum Action {
    UpdatePreedit(String),
    Commit(String),
    PassThrough,
}
```

[Suy luận] Tách FSM ra khỏi IBus glue cho phép viết unit test mà không cần chạy ibus-daemon — quan trọng cho chất lượng.

### 3.4. Vietnamese syllable rules

Bảng dữ liệu tĩnh mô tả:

- Tập nguyên âm hợp lệ tiếng Việt
- Vị trí đặt dấu thanh (theo quy tắc chính tả)
- Phụ âm đầu / phụ âm cuối hợp lệ

[Chưa xác minh] Có nhiều quy ước về vị trí đặt dấu ("kiểu cũ" vs "kiểu mới" — ví dụ "hoà" vs "hòa"). Cần quyết định mặc định và cho phép cấu hình.

---

## 4. Cấu trúc thư mục (tổng quát)

```
ibus-vie/
├── Cargo.toml                # workspace root
├── crates/
│   ├── ibus-vie-im/            # FSM thuần — KHÔNG depend IBus
│   ├── ibus-vie-vi/            # Quy tắc âm tiết tiếng Việt
│   ├── ibus-vie-engine/        # IBus binary (giao tiếp DBus)
│   └── ibus-vie-cli/           # Tool debug FSM ở terminal
├── data/                     # vie.xml.in, icons
├── tests/snapshot/           # Test case dạng text, dễ contribute
├── packaging/                # debian/, rpm/, arch/
├── docs/
└── Makefile                  # Wrapper tiện ích quanh cargo
```

**Chi tiết đầy đủ về layout, ranh giới module, dependency graph, cách debug → xem `SOURCE_LAYOUT.md`.**

---

## 5. Cách `ibus-vie` đến được Settings

[Suy luận] Quy trình mà repo này dựa vào:

1. Cài package → file `vie.xml` được đặt tại `/usr/share/ibus/component/`.
2. Trigger IBus đọc lại component (qua `ibus write-cache --system` hoặc reload).
3. GNOME Settings (gnome-control-center) liệt kê các engine từ component này.
4. Người dùng bấm Add → entry "Vietnamese (ibus-vie — Telex)" xuất hiện.
5. Khi người dùng chọn input source này, IBus gọi `exec` trong XML → engine binary chạy.
6. Engine binary đăng ký với `ibus-daemon` qua DBus.
7. Mutter routes phím nhấn của ứng dụng đang focus về engine qua giao thức Wayland input-method → IBus → engine.

[Chưa xác minh] Một số bước (ví dụ có cần `ibus write-cache` hay GNOME tự phát hiện) phụ thuộc phiên bản IBus và GNOME. Cần kiểm chứng khi đóng gói.

---

## 6. Ràng buộc thiết kế

- **R1.** Không tự khởi tạo daemon. Toàn bộ vòng đời do `ibus-daemon` quản lý.
- **R2.** Không sửa file của user (chỉ đọc config nếu có).
- **R3.** Không cần quyền root để chạy. Cần root chỉ để cài.
- **R4.** Phải làm việc với keyboard layout `us` mặc định — không yêu cầu người dùng đổi layout.
- **R5.** Không block khi xử lý phím; tất cả phải đồng bộ và nhanh (< vài ms).
