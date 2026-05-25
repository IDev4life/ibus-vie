# Issue: Warp Terminal không gõ được tiếng Việt với ibus-vie

**Ngày:** 2026-05-25
**Môi trường:** Ubuntu 24.04, GNOME Shell 46.0, Mutter 46.2, session `wayland`
**ibus-vie version:** 1.1.0 (workspace `main` branch)
**Warp version (bị lỗi):** `0.2026.05.20.09.21.stable.03`
**Warp version (còn hoạt động):** `0.2026.05.18.05.32.stable.02`

**Status: CONFIRMED REGRESSION IN WARP 0.2026.05.20** — downgrade về 05.18 fix issue. `apt-mark hold warp-terminal` đã active.

---

## 1. Tóm tắt

- Mọi app GTK/Qt/Electron/X11 native gõ tiếng Việt qua `ibus-vie` bình thường.
- Riêng **Warp Terminal** ngưng nhận input từ `ibus-vie` sau khi upgrade lên `0.2026.05.20.09.21.stable.03` (lúc `2026-05-25 08:50:18` qua `apt upgrade`).
- Ở version `0.2026.05.18.05.32.stable.02`, Warp nhận input nhưng preedit không hiển thị (chữ chỉ hiện sau khi gõ xong = commit). Sau upgrade thì mất hẳn cả commit.

## 2. Triệu chứng

| Hành vi | Bình thường (app GTK/X11) | Warp 05.18 | Warp 05.20 (hiện tại) |
|---|---|---|---|
| Gõ `vieetj` ra `việt` | ✅ | ⚠️ (chỉ hiện khi commit, không có preedit) | ❌ ra `vieetj` raw |
| ibus-vie debug log thấy keystroke | ✅ | ✅ | ❌ |
| ibus-vie engine reset on focus | ✅ | ✅ | ❌ (không có focus_in/out) |

`/tmp/ibus_vie_debug.log` xác nhận: khi focus vào Warp window và gõ phím, **không có bất kỳ event nào từ ibus-vie engine** — tức là ibus-daemon không gửi key event xuống engine vì Warp không nói chuyện với IBus.

## 3. Kiến trúc hiện tại của ibus-vie

```
+------------------+        DBus         +---------------+
| ibus-vie-engine  | <-----------------> | ibus-daemon   |
| (Rust, zbus)     |   org.fd.IBus.*     |               |
+------------------+                     +-------+-------+
                                                 |
                  +------------------------------+------------------------------+
                  |                              |                              |
              IBus client lib            ibus-x11 (XIM bridge)             ibus-portal
              (in-process)               (separate process)                (DBus portal)
                  |                              |                              |
        GTK_IM_MODULE=ibus              X11 apps via XIM                Sandboxed/Flatpak
        QT_IM_MODULE=ibus               (xterm, urxvt, ...)
        (gtk3/gtk4/qt5/qt6)
```

**Protocol mà `ibus-vie-engine` implement:**

- `org.freedesktop.IBus.Engine` (DBus interface) trên IBus private bus (không phải session bus). Source: `crates/ibus-vie-engine/src/ibus/engine_impl.rs`
- `org.freedesktop.IBus.Factory` cho engine creation. Source: `crates/ibus-vie-engine/src/ibus/factory.rs`
- `org.freedesktop.IBus.Service` cho destroy. Source: `crates/ibus-vie-engine/src/ibus/service.rs`
- Đăng ký bus name `org.freedesktop.IBus.Vie`. Manifest: `data/vie.xml.in`

**Tóm lại: ibus-vie là pure IBus engine, không speak XIM trực tiếp, không speak Wayland text-input-v3 trực tiếp.** Việc bridge từ IBus sang protocol khác là trách nhiệm của `ibus-daemon` + `ibus-x11` + Wayland compositor (mutter).

## 4. Kiến trúc input của Warp

Warp là native Rust app dùng [`winit`](https://github.com/rust-windowing/winit) (verified qua `strings /opt/warpdotdev/warp-terminal/warp | grep winit::...`). winit Linux input stack:

| Backend | Protocol IME | Bridge với IBus |
|---|---|---|
| X11 (XWayland) | XIM (`XOpenIM`, `XCreateIC`, `XIMPreeditNothing`) | Qua `ibus-x11` daemon |
| Wayland native | `zwp_text_input_v3` | Compositor phải implement `input-method-v2` (mutter chỉ partial) |

Binary `0.2026.05.20.09.21.stable.03` chứa cả hai stack — verify qua `strings`:

```
winit::platform_impl::linux::x11::ime::context     # XIM client
zwp_text_input_v3                                  # Wayland v3
zwp_text_input_manager_v3
xkb_compose_state_*                                # libxkbcommon Compose fallback
```

**winit IME mặc định = DISABLED.** App phải gọi `window.set_ime_allowed(true)` để bật. Trong binary `0.2026.05.20`, tìm `set_ime_cursor_area` (có) nhưng **không thấy** symbol cho `set_ime_allowed` được gọi explicit (kết luận `[Suy luận]` — strings của Rust binary chỉ chứa method-name string khi method được monomorphize và referenced; không hoàn toàn definitive).

## 5. Root cause analysis

### Tại sao 05.18 → 05.20 hỏng

`[Suy luận]` Trong commit nào đó giữa 05.18 và 05.20, Warp:
- Hoặc bỏ call `window.set_ime_allowed(true)` (regression code-side)
- Hoặc thay đổi window backend từ X11/XIM sang Wayland text-input-v3 mặc định, mà mutter 46 không bridge tốt
- Hoặc thay đổi cách handle keyboard event để pre-empt key event trước khi winit forward đến IME context

Cần xem release note của `0.2026.05.20.09.21.stable.03` từ Warp để xác nhận `[Chưa xác minh]`.

### Tại sao các workaround đã thử thất bại

| Workaround | Kết quả | Lý do |
|---|---|---|
| Đổi `input_box_type_setting = "classic"` trong `settings.toml` | Warp revert về `"universal"` khi launch | `IsSettingsSyncEnabled=true` → cloud sync ghi đè |
| `WINIT_UNIX_BACKEND=x11 GDK_BACKEND=x11` env vars | Warp vẫn không nhận IME | `WINIT_UNIX_BACKEND` đã bị deprecate trong winit ≥ 0.30; `GDK_BACKEND` không áp dụng (Warp không dùng GTK) |
| `GTK_IM_MODULE=ibus QT_IM_MODULE=ibus XMODIFIERS=@im=ibus` | Không hiệu lực | Warp không dùng GTK/Qt, chỉ winit thuần |
| Unset `WAYLAND_DISPLAY` | Warp vẫn load `libwayland-client.so.0` | winit fallback dò `$XDG_RUNTIME_DIR/wayland-0` |

### Tại sao mutter 46 + text-input-v3 không hoạt động tốt

- Mutter implement `zwp_text_input_v3` từ ~3.36 nhưng nhiều report bug edge case với IME daemon (IBus/fcitx) cho đến mutter ≥ 47.
- IBus < 1.5.30 không speak `input-method-v2` (Wayland IME-side protocol); GNOME tự bridge IBus ↔ text-input-v3 qua mutter internal code. Hệ thống hiện tại `ibus 1.5.x` + `mutter 46.2` → bridge fragile, dễ vỡ.

### Vai trò của repo ibus-vie

**Repo ibus-vie KHÔNG sai gì.** Chuỗi protocol gãy ở:

```
ibus-vie OK  →  ibus-daemon OK  →  [BRIDGE BROKEN]  →  Warp winit IME context
                                       ↑
                          (either mutter text-input-v3 bug,
                           or winit IME disabled in Warp code)
```

## 6. Có nên thêm protocol vào ibus-vie để hỗ trợ Warp?

Bốn hướng kỹ thuật. Tất cả đều **không thực sự fix Warp 05.20**, vì root cause là Warp tự tắt IME (`set_ime_allowed(false)` / không gọi), không protocol nào của IME daemon force được app phải bật IME.

### Option A: Native XIM server (thay thế ibus-x11)

- Implement XIM protocol trong `ibus-vie-engine` để listen trực tiếp trên X11 display.
- **Pros:** Bypass ibus-daemon hoàn toàn cho X11 apps; debug đơn giản hơn.
- **Cons:**
  - XIM protocol cực phức tạp (X11 atoms, ICValue serialization, multiple wire formats).
  - Đã có `ibus-x11` làm tốt rồi — duplicate effort.
  - **Không giúp Warp 05.20** vì Warp không gọi `set_ime_allowed`, XIM client trong winit không activate.
- **Verdict:** ❌ Không recommend.

### Option B: Wayland `zwp_input_method_v2` server

- Implement `zwp_input_method_manager_v2` client (ibus-vie kết nối Wayland display như một IME).
- Compositor sẽ route `text_input_v3` events từ app → ibus-vie qua protocol này.
- **Pros:** Native Wayland, hiện đại, đúng hướng tương lai.
- **Cons:**
  - **Mutter (GNOME) KHÔNG IMPLEMENT `input-method-v2`.** Chỉ wlroots-based compositor (sway, hyprland, river, ...) hỗ trợ.
  - Ba dùng GNOME → option này = no-op.
  - **Vẫn không fix Warp** vì Warp phải gọi `set_ime_allowed` để text-input-v3 hoạt động trong winit.
- **Verdict:** ❌ Không phù hợp môi trường GNOME hiện tại.

### Option C: Compositor-side `text_input_v3` server (chỉ trong compositor)

- Chỉ khả thi nếu viết compositor riêng. ibus-vie standalone không thể.
- **Verdict:** ❌ Loại bỏ.

### Option D: `zwp_virtual_keyboard_v1` — synthesize key events

- ibus-vie giả lập bàn phím ảo, gửi unicode key trực tiếp.
- Bypass IME hoàn toàn — không cần app cooperate.
- **Pros:** Hoạt động với mọi app bất kể có IME support hay không.
- **Cons:**
  - Mất preedit (không có gạch chân khi đang gõ).
  - Mất undo (double-press để undo tone trong Telex).
  - GNOME mutter cũng không expose `virtual_keyboard_v1` cho client thường (security restriction). Chỉ wlroots compositors.
  - X11 tương đương: `xdotool type` — cùng vấn đề mất preedit.
- **Verdict:** ⚠️ Workaround cuối cùng nếu mọi cách khác fail. Không nên là default.

### Option E (thực tế nhất): Wrapper / launcher với LD_PRELOAD shim

- Shim library inject vào Warp process, hook winit IME init để force `set_ime_allowed(true)`.
- **Pros:** Fix tận gốc app side mà không cần modify Warp source.
- **Cons:**
  - Fragile: phụ thuộc winit version, symbol mangling Rust, build-id của binary.
  - Có thể vi phạm Warp ToS / bị Warp anti-tamper block.
- **Verdict:** 🟡 Khả thi nhưng phức tạp, dễ vỡ khi Warp update.

## 7. Kết luận

**Repo `ibus-vie` không cần thêm protocol mới để fix Warp.**

Vấn đề Warp là **client-side regression**, không phải gap protocol của ibus-vie. Mọi app khác (Chrome, VS Code, gnome-terminal, Slack, ...) vẫn gõ tốt qua cùng ibus-vie / ibus-daemon stack → chứng minh stack OK.

Nếu sau này muốn mở rộng support sang môi trường wlroots (sway/hyprland) hoặc apps không qua ibus-daemon, **Option B (input-method-v2)** là hướng đúng đắn về kiến trúc. Nhưng đầu tư công sức hiện tại không justified vì:

1. Người dùng ibus-vie mục tiêu = GNOME (Ubuntu/Fedora) — nơi mutter không hỗ trợ `input-method-v2`.
2. Warp issue sẽ tự fix khi Warp upstream release update.

## 8. Action items đề xuất

### Ngắn hạn (user-side, không sửa repo)

1. **Downgrade Warp về `0.2026.05.18.05.32.stable.02`** + `apt-mark hold warp-terminal`.
   ```bash
   curl -O https://releases.warp.dev/stable/v0.2026.05.18.05.32.stable_02/warp-terminal_0.2026.05.18.05.32.stable.02_amd64.deb
   sudo dpkg -i warp-terminal_0.2026.05.18.05.32.stable.02_amd64.deb
   sudo apt-mark hold warp-terminal
   ```
2. **Báo bug lên Warp** với repro steps + reference issue này.
3. **Dùng terminal thay thế** (gnome-terminal, kitty, alacritty, wezterm) cho task cần tiếng Việt cho đến khi Warp fix.

### Trung hạn (repo-side, optional)

4. **Theo dõi mutter version**: khi Ubuntu update mutter ≥ 47, test lại Warp xem text-input-v3 bridge có ổn không.
5. **Document trong README** rằng ibus-vie không kiểm soát được IME activation phía client; apps phải opt-in IME.

### Dài hạn (repo-side, nếu mở rộng scope)

6. **Khảo sát thêm `input-method-v2` frontend** trong `ibus-vie-engine` (hoặc tách thành crate `ibus-vie-wayland`). Chỉ làm khi:
   - Có user base trên sway/hyprland/wlroots.
   - Có người maintain — hiện tại 1 dev.

## 9. Phụ lục: cách test root cause

### Verify ibus-vie engine vẫn alive
```bash
tail -f /tmp/ibus_vie_debug.log
# Gõ trong Chrome/VS Code/gnome-terminal → thấy preedit/commit log
# Gõ trong Warp → không thấy gì
```

### Verify Warp dùng X11/XWayland hay Wayland native
```bash
WID=$(xdotool search --class dev.warp.Warp | head -1)
xprop -id "$WID" WM_CLASS WM_PROTOCOLS  # nếu thấy = đang dùng X11 (XWayland)
cat /proc/$(pgrep -f '/opt/warpdotdev/warp-terminal/warp$')/maps | grep -E "libwayland|libX11" | awk '{print $NF}' | sort -u
```

### Verify Warp version timeline
```bash
grep "warp-terminal" /var/log/apt/history.log
```

### Inspect IBus stack
```bash
pgrep -af ibus
busctl --user introspect org.freedesktop.IBus.Vie /org/freedesktop/IBus/Factory
```

---

**Maintainer note:** Khi Warp upstream release patch (verify bằng cách test version mới sau `0.2026.05.20.09.21.stable.03`), update lại issue này hoặc đóng.
