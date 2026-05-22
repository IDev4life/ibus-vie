# CONTRIBUTING — Hướng dẫn đóng góp

[Chưa xác minh] Repo hiện chưa có maintainer chính thức và quy trình review. Tài liệu này phác thảo cách làm việc khi repo đi vào hoạt động.

---

## Đóng góp loại nào được hoan nghênh

- **Báo lỗi gõ sai** — đặc biệt với từ tiếng Việt cụ thể. Kèm: kiểu gõ (Telex/VNI/VIQR), chuỗi phím gõ, kết quả thực tế, kết quả mong đợi.
- **Test case bổ sung** cho `INPUT_METHODS.md`.
- **Fix cụ thể** với PR nhỏ, dễ review.
- **Port packaging** sang distro chưa có.

[Suy luận] PR refactor lớn không được khuyến khích ở giai đoạn đầu vì kiến trúc còn chưa ổn định.

---

## Quy trình PR đề xuất

1. Mở issue trước khi viết code lớn — để tránh trùng việc và lệch hướng.
2. Một PR = một thay đổi logic. Không nhét nhiều fix vào một PR.
3. Mọi thay đổi trong `src/im/` (FSM) phải kèm test.
4. Mọi thay đổi luật chính tả phải có ít nhất một test case trong commit message.

---

## Code style

Ngôn ngữ: **Rust** (edition 2021).

- Format: `cargo fmt --all` — phải pass trước khi mở PR.
- Lint: `cargo clippy --all-targets --all-features -- -D warnings` — không cho phép warning.
- Test: `cargo test --workspace` phải pass.
- Tài liệu hàm public: `cargo doc --no-deps` không được warning về missing docs trên các API public.

[Suy luận] Quy ước "deny warnings" làm tăng chi phí review một chút nhưng giữ source clean và tránh nợ kỹ thuật tích lũy — phù hợp khi project còn nhỏ.

Đặt tên:

- crate: `ibus-vie-<role>` (vd `ibus-vie-im`, `ibus-vie-engine`).
- module: snake_case.
- type / trait: UpperCamelCase.
- const: SCREAMING_SNAKE_CASE.
- File test snapshot: `tests/snapshot/<method>.txt` với format `input<TAB>expected` mỗi dòng.

---

## Cách báo lỗi gõ tốt

Mẫu issue tốt:

```
Title: Telex: gõ "khoẻ" không ra đúng

Kiểu gõ: Telex
Chuỗi phím: k h o e r
Kết quả thực tế: khỏe
Kết quả mong đợi: khoẻ (kiểu đặt dấu cũ)
Distro: Ubuntu 24.04, GNOME Wayland
Phiên bản ibus-vie: 0.x.y
```

[Suy luận] Issue có chuỗi phím cụ thể dễ debug hơn nhiều so với "gõ không đúng".

---

## Code of Conduct

[Chưa xác minh] Chưa có CoC chính thức. Đề xuất adopt Contributor Covenant khi repo hoạt động thực sự.

---

## Liên hệ

[Chưa xác minh] Chưa có kênh liên hệ chính thức. Tạm thời dùng GitHub Issues.