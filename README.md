# ibus-vie

Bộ gõ tiếng Việt cho Linux, thiết kế **Wayland-first**, tích hợp trực tiếp vào hệ thống thông qua IBus. Mục tiêu trải nghiệm: người dùng chỉ cần mở _Settings → Keyboard → Input Sources_, bấm **+**, và thêm **"Vietnamese (ibus-vie)"** — không cần cài daemon riêng, không cần chỉnh `GTK_IM_MODULE`, không cần chạy lệnh sau đăng nhập.

---

## Trạng thái dự án

**Giai đoạn: Phase 0 → Phase 1** — FSM cho cả 3 kiểu gõ (Telex, VNI, VIQR) đã hoạt động. IBus engine binary biên dịch được. CLI debug tool sẵn sàng. Chưa kiểm thử tích hợp thực tế với `ibus-daemon` trên desktop.

---

## Thử nhanh

```bash
# Build
cargo build --release

# Debug FSM không cần IBus
cargo run -p ibus-vie-cli -- --method telex --input "vieetj"
# → việt

cargo run -p ibus-vie-cli -- --method telex --input "tieengs"
# → tiếng

cargo run -p ibus-vie-cli -- --method vni --input "d9a6u"
# → đâu

cargo run -p ibus-vie-cli -- --method viqr --input "Vie^.t"
# → Việt

# Chạy test
cargo test --workspace

# Lint
cargo clippy --all-targets --all-features -- -D warnings
```

---

## Vì sao tồn tại

Trên Linux Wayland, các bộ gõ tiếng Việt phổ biến hiện nay (ibus-bogo, ibus-unikey, fcitx5-unikey, fcitx5-bamboo) đều hoạt động được, nhưng người dùng mới thường gặp vướng:

- Phải cài thêm framework (fcitx5) hoặc cài thêm package ngoài kho chính.
- Phải đặt biến môi trường `GTK_IM_MODULE`, `QT_IM_MODULE`, `XMODIFIERS` — vốn là khái niệm thừa hưởng từ thời X11.
- Phải tự khởi động daemon, tự thêm vào autostart.

Với người dùng cuối, trải nghiệm lý tưởng là: cài một package → mở Settings → thêm input source → gõ được ngay. Trên GNOME Wayland, đây chính là vòng đời mà các IBus engine đã đăng ký đi qua một cách tự nhiên.

---

## Kiến trúc

```
ibus-vie-cli  ──►  ibus-vie-im  ──►  ibus-vie-vi
ibus-vie-engine ──►  ibus-vie-im
                 ──►  zbus, tokio, tracing, ...
```

| Crate             | Loại | Mục đích                                                       |
| ----------------- | ---- | -------------------------------------------------------------- |
| `ibus-vie-vi`     | lib  | Dữ liệu + luật âm tiết tiếng Việt. Const tables, đặt dấu thanh |
| `ibus-vie-im`     | lib  | FSM Telex/VNI/VIQR thuần. Không I/O. Không IBus                |
| `ibus-vie-engine` | bin  | Binary chạy bởi `ibus-daemon`. Tất cả DBus/IBus glue ở đây     |
| `ibus-vie-cli`    | bin  | Tool dev. Gõ vào terminal → in ra kết quả. Không cần IBus chạy |

**Quy tắc vàng:** logic gõ (`ibus-vie-im`, `ibus-vie-vi`) không được biết IBus tồn tại.

---

## Hướng tiếp cận kỹ thuật

`ibus-vie` được thiết kế như một **IBus engine** viết bằng **Rust**:

1. GNOME Settings → Keyboard → Input Sources liệt kê các IBus engine đã đăng ký mà không cần thêm bước cấu hình.
2. IBus đã được tích hợp sẵn với Mutter (compositor GNOME) qua giao thức `input-method` của Wayland.
3. Không cần daemon thứ hai song song với `ibus-daemon` đã có sẵn.

Giao tiếp với `ibus-daemon` qua DBus bằng crate `zbus` (Rust thuần) — không cần `libibus` C bindings.

---

## Tài liệu

| File                    | Nội dung                                                  |
| ----------------------- | --------------------------------------------------------- |
| `docs/SPEC.md`          | Đặc tả chức năng (goals / non-goals / hành vi)            |
| `docs/ARCHITECTURE.md`  | Kiến trúc kỹ thuật, các thành phần                        |
| `docs/SOURCE_LAYOUT.md` | **Cấu trúc Cargo workspace, ranh giới crate, cách debug** |
| `docs/INPUT_METHODS.md` | Luật gõ Telex / VNI / VIQR                                |
| `docs/INSTALL.md`       | Hướng dẫn build và cài đặt                                |
| `docs/WAYLAND.md`       | Ghi chú về Wayland & IBus                                 |
| `docs/ROADMAP.md`       | Lộ trình phát triển theo giai đoạn                        |
| `CONTRIBUTING.md`       | Hướng dẫn đóng góp                                        |

---

## License

GPL-3.0-or-later
