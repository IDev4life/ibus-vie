# ibus-vie

Bộ gõ tiếng Việt cho Linux, thiết kế **Wayland-first**, tích hợp trực tiếp vào hệ thống thông qua IBus. Mục tiêu trải nghiệm: người dùng chỉ cần mở _Settings → Keyboard → Input Sources_, bấm **+**, và thêm **"Vietnamese (ibus-vie)"** — không cần cài daemon riêng, không cần chỉnh `GTK_IM_MODULE`, không cần chạy lệnh sau đăng nhập.

---

## Trạng thái dự án

[Chưa xác minh] Đây là kho tài liệu đặc tả ở giai đoạn thiết kế. Chưa có bản cài đặt thực tế của `ibus-vie` nào được kiểm thử. Mọi mô tả hành vi trong tài liệu này là dự định thiết kế, không phải mô tả phần mềm đã chạy.

---

## Vì sao tồn tại

Trên Linux Wayland, các bộ gõ tiếng Việt phổ biến hiện nay (ibus-bogo, ibus-unikey, fcitx5-unikey, fcitx5-bamboo) đều hoạt động được, nhưng người dùng mới thường gặp vướng:

- Phải cài thêm framework (fcitx5) hoặc cài thêm package ngoài kho chính.
- Phải đặt biến môi trường `GTK_IM_MODULE`, `QT_IM_MODULE`, `XMODIFIERS` — vốn là khái niệm thừa hưởng từ thời X11.
- Phải tự khởi động daemon, tự thêm vào autostart.
- [Chưa xác minh] Một số ứng dụng GTK4/Qt6 chạy Wayland thuần có thể ứng xử khác nhau giữa các framework.

[Suy luận] Với người dùng cuối, trải nghiệm lý tưởng là: cài một package → mở Settings → thêm input source → gõ được ngay. Trên GNOME Wayland, đây chính là vòng đời mà các IBus engine đã đăng ký đi qua một cách tự nhiên.

---

## Hướng tiếp cận kỹ thuật

[Suy luận] `ibus-vie` được thiết kế như một **IBus engine** viết bằng **Rust** vì các lý do sau:

1. GNOME Settings → Keyboard → Input Sources liệt kê các IBus engine đã đăng ký mà không cần thêm bước cấu hình. Khi file mô tả engine (XML) được đặt đúng chỗ trong `/usr/share/ibus/component/`, engine xuất hiện trong danh sách.
2. IBus đã được tích hợp sẵn với Mutter (compositor GNOME) qua giao thức `input-method` của Wayland — không cần người dùng tự cấu hình lớp IM.
3. Không cần daemon thứ hai song song với `ibus-daemon` đã có sẵn của GNOME.

[Chưa xác minh] KDE Plasma 6 Wayland cũng tích hợp với IBus, nhưng các chi tiết tích hợp (ví dụ độ ổn định của text-input-v3) cần được kiểm chứng trên từng phiên bản cụ thể trước khi cam kết.

[Suy đoán] Cách tiếp cận này _có thể_ mang lại trải nghiệm đơn giản hơn cho người dùng GNOME so với cài fcitx5 trên môi trường vốn dùng IBus mặc định. Đây là giả thuyết thiết kế, chưa có dữ liệu so sánh.

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

[Chưa xác minh] Chưa chọn license. Đề xuất GPL-3.0 hoặc MIT — quyết định khi tạo repo thực sự.
