# INSTALL — Hướng dẫn build và cài đặt

---

## 1. Yêu cầu hệ thống

### Runtime

- Linux (Wayland hoặc X11).
- IBus đã cài và chạy.
  - GNOME: mặc định có sẵn.
  - KDE: thường phải cài thêm `ibus`.

### Build dependencies

ibus-vie được viết bằng **Rust**. Cần:

- `rustc` + `cargo` (≥ 1.95, edition 2021). Khuyên dùng [rustup](https://rustup.rs/) nếu distro có Rust cũ.
- `make`.

Không cần `pkg-config` hay `ibus` development headers — `ibus-vie` giao tiếp qua DBus bằng crate `zbus` (Rust thuần).

### Tên package theo distro

| Distro        | Lệnh cài deps                       |
| ------------- | ----------------------------------- |
| Ubuntu/Debian | `sudo apt install rustc cargo make` |
| Fedora        | `sudo dnf install rust cargo make`  |
| Arch          | `sudo pacman -S rust make`          |

Nếu phiên bản `rustc` trong kho APT/DNF cũ hơn 1.95, dùng `rustup` để cài toolchain mới.

---

## 2. Build từ source

```bash
git clone https://github.com/IDev4life/ibus-vie.git
cd ibus-vie
cargo build --release
# hoặc
make build
```

Binary đầu ra:

- `target/release/ibus-vie-engine` — IBus engine binary
- `target/release/ibus-vie-cli` — CLI debug tool

---

## 3. Cài đặt

### Cài hệ thống (cần root)

```bash
sudo make install
```

Lệnh này sẽ:

1. Copy binary tới `/usr/libexec/ibus-engine-vie`.
2. Copy CLI tới `/usr/bin/ibus-vie-cli`.
3. Sinh `vie.xml` từ template `data/vie.xml.in` và đặt tại `/usr/share/ibus/component/`.
4. Chạy `ibus write-cache --system` để IBus đọc lại danh sách component.

### Cài cho user (không cần root — dùng để dev/test)

```bash
make install-user
```

Sinh `vie.xml` trỏ thẳng tới binary trong `target/release/` và đặt tại `~/.local/share/ibus/component/`. Sau đó chạy `ibus restart`.

---

## 4. Kích hoạt sau khi cài

### Bước 1. Restart IBus

```bash
ibus restart
```

### Bước 2. Mở Settings

- **GNOME:** Settings → Keyboard → Input Sources
- **KDE Plasma:** System Settings → Virtual Keyboard / Input Method

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

## 5. Cấu hình

File: `~/.config/ibus-vie/config.toml` (tạo nếu cần, không bắt buộc — defaults đủ dùng):

```toml
method = "telex"       # telex | vni | viqr
tone_style = "new"     # new (hòa) | old (hoà)
```

---

## 6. Gỡ cài đặt

```bash
sudo make uninstall
ibus restart
```

---

## 7. Đóng gói cho distro

[Suy đoán] Kế hoạch:

- Debian/Ubuntu package (`.deb`) — metadata đã sẵn trong `Cargo.toml` (`[package.metadata.deb]`)
- RPM spec cho Fedora/openSUSE
- PKGBUILD cho Arch Linux / AUR

---

## 8. Troubleshooting

### "ibus-vie không hiện trong Settings"

```bash
ibus list-engine | grep vie    # kiểm tra engine có được IBus thấy
ibus restart                   # restart daemon
```

Nếu vẫn không thấy, kiểm tra file XML:

- Hệ thống: `/usr/share/ibus/component/vie.xml`
- User: `~/.local/share/ibus/component/vie.xml`

### "Engine hiện trong Settings nhưng gõ không ra tiếng Việt"

```bash
# Kiểm tra binary chạy được
/usr/libexec/ibus-engine-vie --version

# Xem log
RUST_LOG=ibus_vie=debug ibus restart
journalctl --user -f | grep ibus-vie
```

### "Gõ được ở GTK app nhưng không ở Firefox / Electron"

Một số ứng dụng XWayland có thể cần biến môi trường legacy. Đây là vấn đề chung của Linux IM stack, không riêng ibus-vie.

### Debug FSM (không cần IBus)

```bash
cargo run -p ibus-vie-cli -- --method telex --input "vieetj"
# → việt

cargo run -p ibus-vie-cli -- --method telex --trace --input "vieetj"
# → in từng bước FSM
```
