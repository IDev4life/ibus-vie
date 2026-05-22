# CONTRIBUTING — Hướng dẫn đóng góp

---

## Đóng góp loại nào được hoan nghênh

- **Báo lỗi gõ sai** — đặc biệt với từ tiếng Việt cụ thể. Kèm: kiểu gõ (Telex/VNI/VIQR), chuỗi phím gõ, kết quả thực tế, kết quả mong đợi.
- **Test case bổ sung** — thêm dòng vào `tests/snapshot/telex.txt` (hoặc `vni.txt`, `viqr.txt`). Không cần biết Rust.
- **Fix cụ thể** với PR nhỏ, dễ review.
- **Port packaging** sang distro chưa có.

PR refactor lớn không được khuyến khích ở giai đoạn đầu vì kiến trúc còn đang ổn định.

---

## Quy trình PR đề xuất

1. Mở issue trước khi viết code lớn — để tránh trùng việc và lệch hướng.
2. Một PR = một thay đổi logic. Không nhét nhiều fix vào một PR.
3. Mọi thay đổi trong `crates/ibus-vie-im/` (FSM) phải kèm test.
4. Mọi thay đổi luật chính tả phải có ít nhất một test case trong snapshot file hoặc commit message.

---

## Build & Test

```bash
cargo build --release          # build tất cả
cargo test --workspace         # chạy tất cả test
cargo fmt --all                # format
cargo clippy --all-targets --all-features -- -D warnings  # lint
make check                     # fmt + lint + test (shortcut)

# Debug FSM nhanh (không cần IBus)
cargo run -p ibus-vie-cli -- --method telex --input "vieetj"

# Interactive mode — gõ trực tiếp, Ctrl+D thoát
cargo run -p ibus-vie-cli -- --method telex

# Trace FSM steps
cargo run -p ibus-vie-cli -- --method telex --trace --input "vieetj"
```

---

## Code style

Ngôn ngữ: **Rust** (edition 2021, MSRV 1.95).

- Format: `cargo fmt --all` — phải pass trước khi mở PR.
- Lint: `cargo clippy --all-targets --all-features -- -D warnings` — zero warning policy.
- Test: `cargo test --workspace` phải pass.

Đặt tên:

- crate: `ibus-vie-<role>` (vd `ibus-vie-im`, `ibus-vie-engine`).
- module: snake_case.
- type / trait: UpperCamelCase.
- const: SCREAMING_SNAKE_CASE.
- File test snapshot: `tests/snapshot/<method>.txt` với format `input<TAB>expected` mỗi dòng.

---

## Thêm test case

Cách dễ nhất để đóng góp — chỉ cần thêm dòng vào file snapshot:

```
# tests/snapshot/telex.txt
vieetj	việt
tieengs	tiếng
```

Format: `chuỗi_phím<TAB>kết_quả_mong_đợi`. Mỗi dòng một test case. Dòng bắt đầu bằng `#` là comment.

Sau khi thêm, chạy `cargo test --workspace` để verify.

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

Issue có chuỗi phím cụ thể dễ debug hơn nhiều so với "gõ không đúng".

---

## Liên hệ

Tạm thời dùng GitHub Issues.
