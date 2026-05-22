# WAYLAND — Ghi chú thiết kế về Wayland

Tài liệu này giải thích cách input method hoạt động trên Wayland và lý do `ibus-vie` chọn con đường IBus engine.

---

## 1. Vì sao Wayland làm khó input method

Trên X11, input method có thể "chen" giữa keyboard và ứng dụng bằng nhiều cơ chế cũ (XIM, GTK_IM_MODULE qua `LD_PRELOAD`-style, v.v.). Bất tiện cho người dùng (phải chỉnh env), nhưng dễ cho dev IM.

Trên Wayland:

- Ứng dụng **không** nói chuyện trực tiếp với input method.
- Compositor là trung gian duy nhất.
- IM nói với compositor qua giao thức Wayland chuyên dụng (text-input và input-method).
- → Không cần biến môi trường, không cần XIM, nhưng IM phải được compositor thừa nhận.

Trên Wayland, **compositor quyết định IM nào được dùng**. Người dùng không thể "tự cài daemon IM bất kỳ rồi mong nó hoạt động" như thời X11.

---

## 2. Hai giao thức Wayland liên quan

### 2.1. `text-input-v3` (ứng dụng ⇄ compositor)

- Ứng dụng (Firefox, GNOME Text Editor, ...) dùng giao thức này để báo cho compositor: "ở đây có một text field đang focus, hãy gửi preedit/commit tới".
- GTK3/4, Qt6 đều có triển khai.

### 2.2. `input-method-v2` (compositor ⇄ input method)

- Compositor dùng giao thức này để giao tiếp với IM client.
- KWin (Plasma 6) hỗ trợ giao thức này.
- Mutter (GNOME) **không** expose giao thức này ra ngoài — Mutter có IBus tích hợp sẵn, IM client phải nói chuyện qua IBus.

Đây là lý do `ibus-vie` chọn con đường IBus engine (xem `ARCHITECTURE.md` §2): GNOME Mutter không cho IM client bên ngoài kết nối trực tiếp như KWin — phải đi qua IBus.

---

## 3. Stack mà ibus-vie dùng

```
App (text-input-v3)
   ↓
Mutter / KWin
   ↓
ibus-daemon
   ↓
ibus-vie engine (DBus session bus, qua zbus 5)
```

Lợi điểm của stack này:

- **Người dùng không cần đặt biến môi trường.** GTK/Qt trên Wayland mặc định dùng text-input-v3 → đi qua compositor → đến ibus-vie.
- **Không cần XWayland.** Ngay cả ứng dụng Wayland-pure (vd Firefox) cũng hoạt động.
- **Không cần `GTK_IM_MODULE=ibus`** trên Wayland thuần — đây là di sản X11.

Trên một số ứng dụng dùng XWayland (vd Electron cũ, JetBrains IDE bản cũ), input method vẫn đi qua đường X11 → IBus. Đây không phải vấn đề của ibus-vie mà là vấn đề chung của stack Linux IM.

---

## 4. Hạn chế và rủi ro đã biết

### 4.1. XWayland apps

Ứng dụng chạy qua XWayland vẫn cần đường XIM/`GTK_IM_MODULE`. Người dùng có thể vẫn cần đặt biến môi trường nếu họ dùng nhiều ứng dụng X11. ibus-vie không tự ép cấu hình này.

### 4.2. KDE Plasma

[Chưa xác minh] KDE Plasma 6 trên Wayland tích hợp IBus, nhưng UI quản lý input method có khác biệt nhỏ so với GNOME. Cần test trên Plasma trước khi tuyên bố hỗ trợ.

### 4.3. Compositor khác (sway, Hyprland, ...)

sway và Hyprland triển khai `input-method-v2` trực tiếp, không qua IBus. ibus-vie trong cấu hình mặc định **có thể không hoạt động** trên các compositor này. Giải pháp:

- Người dùng chạy `ibus-daemon` thủ công và cấu hình compositor dùng nó.
- Hoặc phát triển adapter Wayland input-method-v2 cho ibus-vie ở giai đoạn sau.

Đây không phải mục tiêu v1.

### 4.4. Phiên bản

Hành vi cụ thể của Mutter / KWin / IBus thay đổi qua các phiên bản. Mọi tuyên bố "ibus-vie hoạt động trên GNOME 4x.y" phải kèm số phiên bản kiểm thử.

---

## 5. Quyết định thiết kế then chốt

| Quyết định                                 | Lý do                                                                     |
| ------------------------------------------ | ------------------------------------------------------------------------- |
| Dùng IBus engine, không tự kết nối Wayland | Để xuất hiện trong Settings → Input Sources tự động                       |
| Hỗ trợ Wayland-first                       | Vì đây là tương lai và là môi trường chính của GNOME/KDE hiện đại         |
| Không yêu cầu biến môi trường              | Vì Wayland không cần — đây là điểm khác biệt so với setup fcitx5 thời X11 |
| Không tự khởi tạo daemon                   | Tận dụng `ibus-daemon` của hệ thống                                       |
| Giao tiếp qua `zbus` 5                     | Rust thuần, không cần libibus C bindings                                  |

Tất cả các quyết định trên hướng tới user story: _"mở Settings, add Vietnamese, xong."_
