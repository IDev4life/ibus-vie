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
                          │  IBus integration (Mutter tích hợp sẵn)
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  ibus-daemon (đã có sẵn trong GNOME, hoặc cài thêm trên KDE)│
└─────────────────────────────────────────────────────────────┘
                          ▲
                          │  IBus IPC (DBus session bus)
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  ibus-vie engine                                             │
│  ┌───────────────┐  ┌──────────────┐  ┌─────────────────┐   │
│  │ IBus Engine   │  │ Input Method │  │ Vietnamese      │   │
│  │ glue (zbus)   │──│ FSM          │──│ syllable rules  │   │
│  └───────────────┘  └──────────────┘  └─────────────────┘   │
│  ibus-vie-engine      ibus-vie-im        ibus-vie-vi         │
└─────────────────────────────────────────────────────────────┘
```

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

File template: `data/vie.xml.in`, cài tại `/usr/share/ibus/component/vie.xml`.

```xml
<?xml version="1.0" encoding="utf-8"?>
<component>
  <name>org.freedesktop.IBus.Vie</name>
  <description>Vietnamese input method (ibus-vie)</description>
  <exec>@LIBEXEC@/ibus-engine-vie --ibus</exec>
  <version>0.1.0</version>
  <author>dev1sme</author>
  <license>GPL-3.0-or-later</license>
  <homepage>https://github.com/IDev4life/ibus-vie</homepage>
  <textdomain>ibus-vie</textdomain>

  <engines>
    <engine>
      <name>vie-telex</name>
      <language>vi</language>
      <license>GPL-3.0-or-later</license>
      <author>dev1sme</author>
      <layout>us</layout>
      <longname>Vietnamese (ibus-vie — Telex)</longname>
      <description>Vietnamese Telex input via ibus-vie</description>
      <symbol>VI</symbol>
      <rank>50</rank>
    </engine>
    <engine>
      <name>vie-vni</name>
      <language>vi</language>
      <license>GPL-3.0-or-later</license>
      <author>dev1sme</author>
      <layout>us</layout>
      <longname>Vietnamese (ibus-vie — VNI)</longname>
      <description>Vietnamese VNI input via ibus-vie</description>
      <symbol>VI</symbol>
      <rank>49</rank>
    </engine>
    <engine>
      <name>vie-viqr</name>
      <language>vi</language>
      <license>GPL-3.0-or-later</license>
      <author>dev1sme</author>
      <layout>us</layout>
      <longname>Vietnamese (ibus-vie — VIQR)</longname>
      <description>Vietnamese VIQR input via ibus-vie</description>
      <symbol>VI</symbol>
      <rank>48</rank>
    </engine>
  </engines>
</component>
```

`@LIBEXEC@` được thay thế bởi Makefile tại thời điểm install (mặc định `/usr/libexec`).

### 3.2. Engine executable

Binary chạy khi IBus kích hoạt engine:

- **Ngôn ngữ:** **Rust** (edition 2021, MSRV 1.95). Không phụ thuộc runtime ngoài (`glibc` là đủ), binary nhỏ, hiệu năng tốt, an toàn bộ nhớ.
- **Vị trí cài:** `/usr/libexec/ibus-engine-vie`.
- **Vòng đời:** `ibus-daemon` tự spawn khi engine được activate, terminate khi không cần.

**Giao tiếp với IBus:** Dùng crate `zbus` 5 (Rust thuần) để nói chuyện trực tiếp với `ibus-daemon` qua DBus session bus. Không cần `libibus` C library.

Crate phụ thuộc chính:

| Crate                            | Version | Vai trò                                           |
| -------------------------------- | ------- | ------------------------------------------------- |
| `zbus`                           | 5       | DBus client async, Rust thuần                     |
| `tokio`                          | 1       | Async runtime (current_thread flavor)             |
| `tracing` + `tracing-subscriber` | 0.1/0.3 | Structured logging cho debug                      |
| `serde` + `toml`                 | 1/1.1   | Đọc file config user                              |
| `clap`                           | 4       | CLI parsing (cho `--ibus` flag và `ibus-vie-cli`) |
| `thiserror`                      | 2       | Error types                                       |
| `insta`                          | 1       | Snapshot testing (dev-dependency)                 |

Chi tiết phân bổ source xem **`SOURCE_LAYOUT.md`**.

### 3.3. Input Method FSM (Finite State Machine)

Phần xử lý phím nhấn → preedit → commit. Tách thành crate `ibus-vie-im` thuần (không phụ thuộc IBus) để test offline.

Chi tiết quy tắc xem `INPUT_METHODS.md`.

**API thực tế:**

```rust
pub trait Engine {
    fn key(&mut self, ev: KeyEvent) -> Action;
    fn reset(&mut self);
    fn preedit(&self) -> &str;
    fn feed_str(&mut self, input: &str) -> String; // convenience cho test
}

pub enum Action {
    Update,
    Commit(String),
    PassThrough,
}

pub struct KeyEvent {
    pub char: Option<char>,
    pub backspace: bool,
    pub escape: bool,
}
```

Tách FSM ra khỏi IBus glue cho phép viết unit test mà không cần chạy `ibus-daemon`.

### 3.4. Vietnamese syllable rules

Crate `ibus-vie-vi` — bảng dữ liệu tĩnh:

- Tập nguyên âm / phụ âm hợp lệ tiếng Việt (`alphabet.rs`)
- Cấu trúc âm tiết (`syllable.rs`)
- Vị trí đặt dấu thanh (`tone.rs`) — mặc định kiểu mới ("hòa"), có config để đổi sang kiểu cũ ("hoà")

---

## 4. Cấu trúc thư mục (tổng quát)

```
ibus-vie/
├── Cargo.toml                # workspace root
├── rust-toolchain.toml       # pin stable toolchain
├── Makefile                  # Wrapper tiện ích quanh cargo
├── crates/
│   ├── ibus-vie-vi/            # Quy tắc âm tiết tiếng Việt (leaf, no deps)
│   ├── ibus-vie-im/            # FSM thuần — KHÔNG depend IBus
│   ├── ibus-vie-engine/        # IBus binary (giao tiếp DBus)
│   └── ibus-vie-cli/           # Tool debug FSM ở terminal
├── data/
│   └── vie.xml.in            # IBus component descriptor (template)
├── tests/snapshot/           # Test case dạng text, dễ contribute
│   ├── telex.txt
│   ├── vni.txt
│   └── viqr.txt
└── docs/
```

**Chi tiết đầy đủ về layout, ranh giới module, dependency graph, cách debug → xem `SOURCE_LAYOUT.md`.**

---

## 5. Cách `ibus-vie` đến được Settings

Quy trình:

1. Cài package → file `vie.xml` được đặt tại `/usr/share/ibus/component/`.
2. Chạy `ibus write-cache --system` (hoặc `ibus restart`) để IBus đọc lại danh sách component.
3. GNOME Settings (gnome-control-center) liệt kê các engine từ component này.
4. Người dùng bấm Add → entry "Vietnamese (ibus-vie — Telex)" xuất hiện.
5. Khi người dùng chọn input source này, IBus gọi `exec` trong XML → engine binary chạy.
6. Engine binary đăng ký với `ibus-daemon` qua DBus session bus (sử dụng `zbus`).
7. Mutter routes phím nhấn của ứng dụng đang focus về engine qua IBus.

Cho dev: `make install-user` cài vào `~/.local/share/ibus/component/` để test không cần root.

---

## 6. Ràng buộc thiết kế

- **R1.** Không tự khởi tạo daemon. Toàn bộ vòng đời do `ibus-daemon` quản lý.
- **R2.** Không sửa file của user (chỉ đọc config nếu có).
- **R3.** Không cần quyền root để chạy. Cần root chỉ để cài.
- **R4.** Phải làm việc với keyboard layout `us` mặc định — không yêu cầu người dùng đổi layout.
- **R5.** Không block khi xử lý phím; tất cả phải đồng bộ và nhanh (< vài ms).
