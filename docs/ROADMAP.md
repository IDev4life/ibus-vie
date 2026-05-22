# ROADMAP — Lộ trình phát triển

---

## Phase 0 — Khảo sát kỹ thuật ✅

**Hoàn thành.** Đã xác nhận:

- [x] IBus engine bằng Rust (dùng `zbus` 5) xuất hiện trong GNOME Settings.
- [x] Engine nhận được phím nhấn qua DBus.
- [x] File XML tại `/usr/share/ibus/component/` — cần `ibus write-cache` hoặc `ibus restart`.
- [x] `zbus` (Rust thuần DBus) đủ API để implement IBus engine interface đầy đủ.

---

## Phase 1 — MVP Telex ✅

**Hoàn thành.**

- [x] FSM Telex viết tách rời, có unit test.
- [x] Snapshot test cases pass.
- [x] IBus glue commit preedit đúng cách.
- [x] Backspace, Space, Enter xử lý đúng trong preedit.
- [x] Phím `z` xoá dấu.
- [x] Quy tắc đặt dấu cho nguyên âm đôi/ba (mặc định kiểu mới).
- [x] Cài qua `make install` / `make install-user`.

---

## Phase 2 — VNI ✅

**Hoàn thành.**

- [x] FSM VNI với snapshot tests.
- [x] Component XML mở rộng cho cả 2 engine (vie-telex, vie-vni).
- [x] CLI debug tool hỗ trợ cả 2 method.
- [x] Tài liệu `INPUT_METHODS.md` cập nhật theo behavior thực tế.

---

## Phase 3 — Đóng gói & kiểm thử tích hợp (đang thực hiện)

- [ ] Kiểm thử tích hợp thực tế với `ibus-daemon` trên desktop.
- [ ] Debian package, test trên Ubuntu 24.04+.
- [ ] RPM spec, test trên Fedora hiện hành.
- [ ] PKGBUILD cho Arch / AUR.
- [ ] Test trên nhiều ứng dụng: Firefox, GNOME Text Editor, Terminal, LibreOffice.

---

## Phase 4 — Ổn định v1.0

- [ ] Test trên KDE Plasma Wayland.
- [ ] Fix các lỗi báo cáo trong giai đoạn beta.
- [ ] Mở rộng test suite (100+ từ mỗi method).
- [ ] Tài liệu user-facing.

**Tiêu chí kết thúc:** v1.0.0 release.

---

## Sau v1.0 — các hướng có thể đi

- **Wayland input-method-v2 client trực tiếp** — bypass IBus, hỗ trợ sway/Hyprland/Niri.
- **Tùy biến phím dấu** — cho phép người dùng custom bảng phím Telex.
- **Macro / abbreviations** — gõ tắt cụm từ.
- **Từ điển gợi ý** — gõ "vn" → "Việt Nam".
- **GUI cấu hình** — xem xét khi user base lớn.

---

## Anti-roadmap (sẽ KHÔNG làm)

- Không port sang Windows / macOS.
- Không trở thành framework IM (không cạnh tranh với IBus/fcitx5).
- Không có cloud sync, telemetry, account.
- Không tự khởi tạo daemon riêng.
