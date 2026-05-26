# SPEC — Đặc tả chức năng

Tài liệu này mô tả **ibus-vie làm gì** ở góc độ người dùng và hành vi quan sát được. Không mô tả cách triển khai (xem `ARCHITECTURE.md`).

---

## 1. Goals (Mục tiêu)

### G1. Tích hợp qua Settings, không cần thao tác phụ

Sau khi cài package, người dùng:

1. Mở **Settings → Keyboard → Input Sources** (GNOME) hoặc tương đương trên KDE.
2. Bấm **+ Add an Input Source → Vietnamese → ibus-vie**.
3. Gõ được ngay, không cần chỉnh biến môi trường, không cần restart session.

Engine đăng ký với IBus qua file `vie.xml` và được Mutter nhận diện tự động.

### G2. Wayland-first

Mọi hành vi mặc định được thiết kế cho Wayland. X11 chỉ được hỗ trợ ở mức "không phá vỡ" — không phải mục tiêu chính.

### G3. Hỗ trợ hai kiểu gõ phổ biến

- **Telex** (mặc định)
- **VNI**

Người dùng chuyển kiểu gõ qua menu IBus của panel hoặc shortcut.

### G4. Không phụ thuộc daemon thứ hai

Chỉ dùng `ibus-daemon` đã có sẵn của hệ thống. Không cài fcitx5, không tạo systemd unit riêng cho ibus-vie.

### G5. Có thể tắt nhanh (Toggle)

Mặc định: tổ hợp phím chuyển giữa "gõ tiếng Việt" và "gõ thẳng" do IBus điều khiển. Phím tắt cụ thể tuỳ cấu hình IBus của người dùng — ibus-vie không ép buộc.

---

## 2. Non-goals (Không phải mục tiêu)

- **NG1.** Không tự viết framework IM mới. ibus-vie là **engine**, không phải framework.
- **NG2.** Không hỗ trợ Windows / macOS.
- **NG3.** Không có GUI cấu hình phức tạp. Một file config đơn giản là đủ.
- **NG4.** Không tích hợp từ điển từ ghép / gợi ý từ ở giai đoạn đầu.
- **NG5.** Không nhằm thay thế fcitx5-unikey cho người dùng đã hài lòng với fcitx5.

---

## 3. User stories

### US-1. Người dùng GNOME mới cài Ubuntu

> "Tôi vừa cài Ubuntu, muốn gõ tiếng Việt. Tôi mở Settings, thêm Vietnamese, gõ được. Hết."

[Suy luận] Đây là kịch bản cốt lõi mà thiết kế phải tối ưu.

### US-2. Người dùng Fedora Workstation

> "Tôi cài `ibus-vie` từ COPR / RPM. Mở Settings → Input Sources, thấy 'Vietnamese (ibus-vie)'. Add, gõ ngay."

[Chưa xác minh] Đóng gói cho Fedora cần được kiểm chứng riêng — phụ thuộc cách Fedora hiện tại đóng gói các ibus engine.

### US-3. Người dùng đa kiểu gõ

> "Tôi quen Telex ở máy, VNI ở chỗ làm. Tôi muốn đổi nhanh giữa hai kiểu."

Cách giải quyết: một engine duy nhất "ibus-vie" đăng ký với IBus. Người dùng chuyển kiểu gõ qua IBus property menu (click vào icon trên panel → chọn Telex/VNI). Lựa chọn được lưu vào `~/.config/ibus-vie/config.toml` cho lần khởi động sau.

---

## 4. Hành vi quan sát được

### 4.1. Cửa sổ preedit

- Khi đang gõ một âm tiết chưa hoàn chỉnh (ví dụ đã gõ `viee`), ký tự đang nhập được hiển thị **gạch chân** ở vị trí con trỏ.
- Khi đủ điều kiện hợp lệ (ví dụ thêm `t` → "việt"), preedit được commit và gạch chân biến mất.
- Phím Space hoặc Enter cũng commit preedit hiện tại.

Cách hiển thị preedit (gạch chân vs. nền màu) phụ thuộc vào ứng dụng client — một số ứng dụng GTK/Qt render khác nhau.

### 4.2. Quy tắc commit

- Ký tự không thuộc tập "có thể là tiền tố của một âm tiết tiếng Việt" → commit preedit hiện tại, sau đó xử lý ký tự mới.
- Backspace trong preedit → **xoá thao tác cuối** (không xoá ký tự đã commit) — tức là "huỷ dấu" thay vì xoá chữ. Đây là hành vi quen thuộc của ibus-bogo và unikey.

### 4.3. Phím tắt

| Tổ hợp                                | Hành vi                              |
| ------------------------------------- | ------------------------------------ |
| `Super+Space`                         | (GNOME mặc định) Chuyển input source |
| Phím chuyển IBus do hệ thống cấu hình | Bật/tắt gõ tiếng Việt                |

ibus-vie **không** tự đăng ký phím tắt toàn cục.

---

## 5. Yêu cầu cài đặt (từ góc độ người dùng)

Người dùng cần:

- Linux với GNOME (Wayland) hoặc KDE Plasma (Wayland).
- IBus đã được cài và đang chạy (mặc định trên hầu hết bản phân phối lớn dùng GNOME).
- Cài package `ibus-vie` (qua APT/DNF/Pacman/AUR, tuỳ distro) hoặc build từ source.

Sau khi cài, cần `ibus restart` để IBus nhận component mới.

---

## 6. Đầu ra mong đợi của bản v1

Bản v1.0 phải:

- [x] Hiển thị trong Settings → Input Sources mà không cần thao tác phụ.
- [x] Gõ được Telex chính xác cho bộ test snapshot.
- [x] Gõ được VNI ở mức "cơ bản dùng được".
- [ ] Không crash `ibus-daemon` trong 1 giờ gõ liên tục (chưa test thực tế).
- [ ] Hoạt động trên: Firefox (Wayland), GNOME Text Editor, Terminal, LibreOffice Writer.
- [ ] Có hướng dẫn cài cho ít nhất 1 distro (package).
