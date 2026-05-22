# INSTALL — Hướng dẫn build và cài đặt

[Chưa xác minh] Tài liệu này mô tả **quy trình dự định**. Repo hiện chưa có code, nên các lệnh dưới đây là khung sườn — sẽ được hoàn thiện khi prototype đầu tiên chạy.

---

## 1. Yêu cầu hệ thống

### Bắt buộc
- Linux với kernel hỗ trợ Wayland (≥ 5.x trên thực tế).
- IBus đã cài và chạy.
  - GNOME: mặc định có sẵn.
  - KDE: thường phải cài thêm `ibus`.
- Compositor: Mutter (GNOME) hoặc KWin (KDE Plasma).

### Build dependencies

ibus-vie được viết bằng **Rust**. Cần:

- `rustc` + `cargo` (≥ 1.75, edition 2021).
- `make` (để chạy các target tiện ích `make install`, `make uninstall`).
- `pkg-config`.

[Chưa xác minh] ibus-vie giao tiếp với `ibus-daemon` qua DBus bằng crate `zbus` (Rust thuần) — nên **không** cần `ibus` development headers / C bindings. Quyết định này được mô tả trong `ARCHITECTURE.md` §3.2 và `SOURCE_LAYOUT.md`. Phương án dùng FFI vào libibus chỉ là phương án dự phòng nếu DBus thuần thiếu API.

### Tên package theo distro

| Distro | Lệnh cài deps |
|--------|---------------|
| Ubuntu/Debian | `sudo apt install rustc cargo make pkg-config` |
| Fedora | `sudo dnf install rust cargo make pkgconf-pkg-config` |
| Arch | `sudo pacman -S rust make pkgconf` |

[Chưa xác minh] Phiên bản `rustc` trong kho APT của Ubuntu LTS có thể cũ — nếu vậy dùng `rustup` để cài toolchain mới hơn.

---

## 2. Build từ source

```bash
git clone https://github.com/<owner>/ibus-vie.git
cd ibus-vie
cargo build --release            # build tất cả crates trong workspace
# hoặc
make build                       # wrapper gọi cargo
```

Binary đầu ra: `target/release/ibus-vie-engine` và `target/release/ibus-vie-cli` (tool debug).

[Chưa xác minh] Đường dẫn target có thể đổi nếu cấu hình `[workspace]` chỉ định `target-dir` khác. Xem `SOURCE_LAYOUT.md`.

---

## 3. Cài đặt

### Cài hệ thống (cần root)

```bash
sudo make install
```

Lệnh này (dự kiến) sẽ:

1. Copy binary tới `/usr/libexec/ibus-engine-vie`.
2. Copy `vie.xml` tới `/usr/share/ibus/component/`.
3. Copy icon tới `/usr/share/ibus/icons/` *(nếu có)*.
4. Chạy `ibus write-cache --system` để IBus đọc lại danh sách component.

[Chưa xác minh] Bước (4) có thể không cần thiết trên các bản IBus mới — sẽ kiểm chứng.

### Cài cho user (không cần root)

[Suy đoán] Có thể hỗ trợ cài vào `~/.local/share/ibus/component/` để dev test mà không cần sudo. Cần kiểm chứng IBus có đọc thư mục này không trong từng phiên bản.

---

## 4. Kích hoạt sau khi cài

### Bước 1. Restart IBus (chỉ lần đầu)

```bash
ibus restart
```

[Chưa xác minh] GNOME đôi khi tự phát hiện engine mới mà không cần lệnh này.

### Bước 2. Mở Settings

- **GNOME:** Settings → Keyboard → Input Sources
- **KDE Plasma:** System Settings → Keyboard → Virtual Keyboard *(hoặc Input Method, phụ thuộc phiên bản)*

### Bước 3. Add input source

Bấm **+**, chọn **Vietnamese**, chọn một trong các entry:

- Vietnamese (ibus-vie — Telex)
- Vietnamese (ibus-vie — VNI)
- Vietnamese (ibus-vie — VIQR)

### Bước 4. Chuyển input source

- GNOME mặc định: `Super + Space`.
- Có thể đổi shortcut trong Settings → Keyboard → Keyboard Shortcuts.

### Bước 5. Kiểm thử

Mở GNOME Text Editor, gõ `xin chaof` → mong đợi: `xin chào`.

---

## 5. Gỡ cài đặt

```bash
sudo make uninstall
```

Hoặc thủ công xoá các file đã cài ở mục 3, rồi `ibus restart`.

---

## 6. Đóng gói cho distro

[Suy đoán] Repo sẽ cung cấp:

- `packaging/debian/` — Debian/Ubuntu package files (`debian/control`, `rules`, ...).
- `packaging/rpm/ibus-vie.spec` — RPM spec cho Fedora/openSUSE.
- `packaging/arch/PKGBUILD` — Arch Linux/Manjaro AUR.

Đây là kế hoạch, chưa có sẵn.

---

## 7. Troubleshooting *(dự kiến)*

[Chưa xác minh] Các vấn đề dưới đây là dự đoán dựa trên kinh nghiệm chung với input method trên Linux. Cần kiểm chứng khi có bản chạy thực:

### "Tôi đã cài, nhưng ibus-vie không hiện trong Settings"
- Chạy `ibus list-engine` để kiểm tra engine có được IBus thấy không.
- Restart `ibus-daemon`: `ibus restart`.
- Log out / log in lại.

### "Engine hiện trong Settings nhưng gõ không ra tiếng Việt"
- Kiểm tra binary `/usr/libexec/ibus-engine-vie` có chạy được không (`file`, `ldd`).
- Xem log: `journalctl --user -f` khi đang gõ.

### "Gõ được ở terminal nhưng không gõ được ở Firefox / Electron app"
[Chưa xác minh] Electron và một số ứng dụng XWayland có thể không nhận input method đúng. Đây là vấn đề chung của Linux IM, không riêng ibus-vie.