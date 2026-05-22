# ROADMAP — Lộ trình phát triển

[Suy đoán] Các mốc thời gian dưới đây là phác thảo. Thời gian thực phụ thuộc vào nguồn lực phát triển.

---

## Phase 0 — Khảo sát kỹ thuật (1–2 tuần)

**Mục tiêu:** Xác nhận giả định kỹ thuật trong `ARCHITECTURE.md` và `WAYLAND.md`.

- [ ] Viết một IBus engine "hello world" bằng **Rust** (dùng `zbus` để nói chuyện với `ibus-daemon`). Xác nhận nó xuất hiện trong GNOME Settings.
- [ ] Xác nhận engine nhận được phím nhấn từ Firefox (Wayland) và GNOME Text Editor.
- [ ] Xác nhận behavior khi cài file XML vào `/usr/share/ibus/component/` — có cần `ibus write-cache` không.
- [ ] Test trên Ubuntu (GNOME), Fedora (GNOME), Arch (GNOME), và (nếu có máy) KDE Plasma.
- [ ] [Chưa xác minh] Xác nhận `zbus` (Rust thuần DBus) đủ API để implement IBus engine interface đầy đủ. Nếu không đủ, fall back sang `gtk-rs` + libibus FFI và cập nhật `ARCHITECTURE.md` / `SOURCE_LAYOUT.md`.

**Tiêu chí kết thúc:** Có một bằng chứng chạy được rằng "add input source trong Settings → engine của ta nhận key" hoạt động.

---

## Phase 1 — MVP Telex (2–4 tuần)

**Mục tiêu:** Một bộ gõ Telex tối thiểu, dùng được hàng ngày.

- [ ] FSM Telex viết tách rời, có unit test.
- [ ] 100 test case Telex pass.
- [ ] IBus glue commit preedit đúng cách.
- [ ] Backspace, Space, Enter xử lý đúng trong preedit.
- [ ] Phím `z` xoá dấu.
- [ ] Quy tắc đặt dấu cho nguyên âm đôi/ba (mặc định kiểu mới).
- [ ] Cài qua `make install`, chạy được trên Ubuntu mới nhất.

**Tiêu chí kết thúc:** Tác giả tự dùng ibus-vie làm bộ gõ chính trong 1 tuần mà không phải fall back sang ibus-bogo.

---

## Phase 2 — VNI và VIQR (2 tuần)

- [ ] FSM VNI với 100 test case.
- [ ] FSM VIQR với 50 test case.
- [ ] Component XML mở rộng cho cả 3 engine.
- [ ] Tài liệu `INPUT_METHODS.md` cập nhật theo behavior thực tế.

---

## Phase 3 — Đóng gói (2–3 tuần)

- [ ] Debian package, test trên Ubuntu 24.04+.
- [ ] RPM spec, test trên Fedora hiện hành.
- [ ] PKGBUILD cho Arch / AUR.
- [ ] [Suy đoán] Snap hoặc Flatpak — khả thi không thì kiểm tra; IBus engine khó đóng gói qua sandboxed format. Có thể bỏ qua.

[Chưa xác minh] Khả năng publish lên kho official của distro (Debian, Fedora) phụ thuộc nhiều yếu tố ngoài kỹ thuật — license rõ ràng, sponsor, v.v.

---

## Phase 4 — Ổn định v1.0 (mở)

- [ ] Test trên KDE Plasma Wayland.
- [ ] Fix các lỗi báo cáo trong giai đoạn beta.
- [ ] Tài liệu user-facing (không phải spec dev).
- [ ] Trang web đơn giản (tùy chọn).

**Tiêu chí kết thúc:** v1.0.0 release.

---

## Sau v1.0 — các hướng có thể đi

[Suy đoán] Các ý dưới đây là khả năng, không phải cam kết:

- **Wayland input-method-v2 client trực tiếp** — bypass IBus, hỗ trợ sway/Hyprland/Niri. Đánh đổi: mất tích hợp với Settings → Input Sources.
- **Tùy biến phím dấu** — cho phép người dùng custom bảng phím cho Telex (vd dùng `w` cho dấu mũ thay vì `a`).
- **Macro / abbreviations** — gõ tắt cụm từ. [Suy đoán] có khả năng làm trong scope IBus engine, chưa rõ độ phức tạp.
- **Từ điển gợi ý** — gõ "vn" → "Việt Nam". Đây là tính năng bự, cần thiết kế riêng.
- **GUI cấu hình** — tránh ở giai đoạn đầu (xem `SPEC.md` §2 NG-3), nhưng có thể xem xét khi user base lớn.

---

## Anti-roadmap (sẽ KHÔNG làm)

- Không port sang Windows / macOS.
- Không trở thành framework IM (không cạnh tranh với IBus/fcitx5).
- Không có cloud sync, telemetry, account.
- Không tự khởi tạo daemon riêng.