# SPEC — Đặc tả chức năng

Tài liệu này mô tả **ibus-vie làm gì** ở góc độ người dùng và hành vi quan sát được. Không mô tả cách triển khai (xem `ARCHITECTURE.md`).

---

## 1. Goals (Mục tiêu)

### G1. Tích hợp qua Settings, không cần thao tác phụ
Sau khi cài package, người dùng:
1. Mở **Settings → Keyboard → Input Sources** (GNOME) hoặc tương đương trên KDE.
2. Bấm **+ Add an Input Source → Vietnamese → ibus-vie**.
3. Gõ được ngay, không cần chỉnh biến môi trường, không cần restart session.

[Suy luận] Để đạt được điều này, engine phải đăng ký đúng với IBus và được Mutter/KWin nhận diện qua giao thức Wayland input-method.

### G2. Wayland-first
Mọi hành vi mặc định được thiết kế cho Wayland. X11 chỉ được hỗ trợ ở mức "không phá vỡ" — không phải mục tiêu chính.

### G3. Hỗ trợ ba kiểu gõ phổ biến
- **Telex** (mặc định)
- **VNI**
- **VIQR**

Người dùng chuyển kiểu gõ qua menu IBus của panel hoặc shortcut.

### G4. Không phụ thuộc daemon thứ hai
Chỉ dùng `ibus-daemon` đã có sẵn của hệ thống. Không cài fcitx5, không tạo systemd unit riêng cho ibus-vie.

### G5. Có thể tắt nhanh (Toggle)
Mặc định: tổ hợp phím chuyển giữa "gõ tiếng Việt" và "gõ thẳng" do IBus điều khiển. [Chưa xác minh] Phím tắt cụ thể tuỳ cấu hình IBus của người dùng — ibus-vie không ép buộc.

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

Cách giải quyết đề xuất: hai entry IBus riêng biệt ("Vietnamese (ibus-vie — Telex)" và "Vietnamese (ibus-vie — VNI)"), người dùng add cả hai và đổi qua phím tắt input source của hệ thống.

[Suy luận] Cách này tận dụng cơ chế chuyển nhanh input source sẵn có (`Super+Space` trên GNOME mặc định).

---

## 4. Hành vi quan sát được

### 4.1. Cửa sổ preedit
- Khi đang gõ một âm tiết chưa hoàn chỉnh (ví dụ đã gõ `viee`), ký tự đang nhập được hiển thị **gạch chân** ở vị trí con trỏ.
- Khi đủ điều kiện hợp lệ (ví dụ thêm `t` → "việt"), preedit được commit và gạch chân biến mất.
- Phím Space hoặc Enter cũng commit preedit hiện tại.

[Chưa xác minh] Cách hiển thị preedit (gạch chân vs. nền màu) phụ thuộc vào ứng dụng client. Một số ứng dụng GTK/Qt có thể render khác nhau.

### 4.2. Quy tắc commit
- Ký tự không thuộc tập "có thể là tiền tố của một âm tiết tiếng Việt" → commit preedit hiện tại, sau đó xử lý ký tự mới.
- Backspace trong preedit → **xoá thao tác cuối** (không xoá ký tự đã commit) — tức là "huỷ dấu" thay vì xoá chữ. [Suy luận] Đây là hành vi quen thuộc của ibus-bogo và unikey.

### 4.3. Phím tắt
| Tổ hợp | Hành vi |
|--------|---------|
| `Super+Space` | (GNOME mặc định) Chuyển input source |
| Phím chuyển IBus do hệ thống cấu hình | Bật/tắt gõ tiếng Việt |

ibus-vie **không** tự đăng ký phím tắt toàn cục.

---

## 5. Yêu cầu cài đặt (từ góc độ người dùng)

[Suy luận] Người dùng được kỳ vọng đã có:

- Linux với GNOME (Wayland) hoặc KDE Plasma (Wayland).
- IBus đã được cài và đang chạy (mặc định trên hầu hết bản phân phối lớn dùng GNOME).
- Cài package `ibus-vie` (qua APT/DNF/Pacman/COPR/AUR, tuỳ distro).

Sau khi cài, restart `ibus-daemon` *có thể* cần thiết. [Chưa xác minh] GNOME thường tự khởi động lại engine khi phát hiện component mới — nhưng không chắc đúng trong mọi phiên bản.

---

## 6. Đầu ra mong đợi của bản v1

Bản v1.0 phải:

- [ ] Hiển thị trong Settings → Input Sources mà không cần thao tác phụ.
- [ ] Gõ được Telex chính xác cho 100% trường hợp trong bộ test cơ bản (xem `INPUT_METHODS.md`).
- [ ] Gõ được VNI và VIQR ở mức "cơ bản dùng được".
- [ ] Không crash `ibus-daemon` trong 1 giờ gõ liên tục (test thủ công).
- [ ] Hoạt động trên: Firefox (Wayland), GNOME Text Editor, Terminal, LibreOffice Writer.
- [ ] Có hướng dẫn cài cho ít nhất 1 distro.

[Suy đoán] Các tiêu chí này khả thi với một engine IBus viết bằng Rust (giao tiếp DBus qua `zbus`), nhưng cần được kiểm chứng bằng prototype trước khi cam kết. Lý do chọn Rust: hiệu năng, không cần runtime, ít phụ thuộc — phù hợp với một component long-running được `ibus-daemon` quản lý vòng đời.