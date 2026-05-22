# SOURCE_LAYOUT — Cấu trúc source code

Tài liệu này mô tả **cách source được phân bổ**, ranh giới giữa các phần, và cách debug. Mục tiêu: ai mới đọc repo trong 10 phút phải biết "code FSM nằm ở đâu", "code DBus nằm ở đâu", "test gõ một từ thì sửa file nào".

Ngôn ngữ: **Rust** (edition 2021, MSRV 1.95). Tổ chức: **Cargo workspace** với nhiều crate nhỏ.

---

## 1. Sơ đồ tổng quát

```
ibus-vie/
├── Cargo.toml                       # [workspace] manifest
├── Cargo.lock
├── rust-toolchain.toml              # pin stable toolchain
├── Makefile                         # wrapper: make build / install / test
├── README.md
├── CONTRIBUTING.md
├── LICENSE
│
├── crates/
│   ├── ibus-vie-im/                   # ① FSM thuần (no IBus, no DBus, dùng vi crate)
│   ├── ibus-vie-engine/               # ② IBus binary (DBus + glue)
│   └── ibus-vie-cli/                  # ③ Dev tool: gõ ở terminal
│
├── data/
│   └── vie.xml.in                     # IBus component descriptor (template)
│
├── tests/
│   └── snapshot/                      # Test case dạng text (xem §6)
│       ├── telex.txt
│       └── vni.txt
│
└── docs/
    ├── SPEC.md
    ├── ARCHITECTURE.md
    ├── SOURCE_LAYOUT.md             # ← bạn đang đọc file này
    ├── INPUT_METHODS.md
    ├── INSTALL.md
    ├── WAYLAND.md
    └── ROADMAP.md
```

---

## 2. Ba crate — ranh giới rõ

| #   | Crate             | Loại | Depend gì                    | Mục đích                                                        |
| --- | ----------------- | ---- | ---------------------------- | --------------------------------------------------------------- |
| ①   | `ibus-vie-im`     | lib  | chỉ `vi` (crates.io)         | FSM Telex/VNI thuần. Không I/O. Không IBus.                     |
| ②   | `ibus-vie-engine` | bin  | `ibus-vie-im` + `zbus` + ... | Binary chạy bởi `ibus-daemon`. Tất cả DBus/IBus glue ở đây.     |
| ③   | `ibus-vie-cli`    | bin  | `ibus-vie-im` + `clap`       | Tool dev. Gõ vào terminal → in ra kết quả. Không cần IBus chạy. |

Quy tắc vàng: **logic gõ không được biết IBus tồn tại**. `ibus-vie-im` phải compile và test được trên một máy không có IBus, không có DBus, không có GUI. Nếu một ngày con đường IBus engine không còn phù hợp, chỉ phải viết lại `ibus-vie-engine`; `ibus-vie-im` không đổi.

### Dependency graph

```
ibus-vie-cli  ────────┐
                    ▼
                ibus-vie-im
                    │
                    ▼
                vi (crates.io)

ibus-vie-engine ──► ibus-vie-im
              ─► zbus, tokio, tracing, serde, toml
```

`ibus-vie-im` depend duy nhất `vi` crate (Vietnamese text transformation, MIT license). Không async, không I/O.

---

## 3. Chi tiết từng crate

### 3.1. `crates/ibus-vie-im/` — FSM gõ

```
crates/ibus-vie-im/
├── Cargo.toml
└── src/
    ├── lib.rs               # pub use các engine
    ├── action.rs            # enum Action, struct KeyEvent
    ├── engine.rs            # trait Engine
    ├── telex.rs             # TelexEngine impl Engine
    ├── vni.rs               # VniEngine impl Engine
    └── buffer.rs            # preedit buffer (wraps vi::IncrementalBuffer)
```

Mọi Vietnamese text transformation (đặt dấu, mark, undo) được delegate cho `vi` crate.
Engines chỉ là thin wrapper: push char vào `vi::IncrementalBuffer`, xử lý commit/backspace/escape.

**API cốt lõi** (`engine.rs`):

```rust
pub trait Engine {
    /// Đẩy một phím vào engine. Trả về Action mô tả việc cần làm.
    fn key(&mut self, ev: KeyEvent) -> Action;

    /// Reset preedit (khi focus chuyển, khi user gõ Esc, ...).
    fn reset(&mut self);

    /// Lấy preedit hiện tại để hiển thị.
    fn preedit(&self) -> &str;

    /// Feed full string, return committed + preedit (convenience cho test).
    fn feed_str(&mut self, input: &str) -> String;
}

pub enum Action {
    Update,
    Commit(String),
    PassThrough,
}
```

Trait `Engine` chính là **điểm tách**. `ibus-vie-engine` chỉ làm việc với `dyn Engine` — không quan tâm Telex hay VNI. Thêm kiểu gõ mới = thêm một file impl `Engine`, không sửa engine binary.

### 3.2. `crates/ibus-vie-engine/` — IBus binary

```
crates/ibus-vie-engine/
├── Cargo.toml
└── src/
    ├── main.rs              # entry: parse CLI, init log, chạy event loop
    ├── ibus/
    │   ├── mod.rs
    │   ├── connection.rs    # mở session DBus tới ibus-daemon
    │   ├── factory.rs       # implement IBus EngineFactory interface
    │   └── engine_impl.rs   # implement IBus Engine interface (process_key_event, ...)
    ├── config.rs            # đọc ~/.config/ibus-vie/config.toml
    ├── log.rs               # cấu hình tracing
    └── error.rs             # enum Error + thiserror
```

**Flow chính của `main.rs`:**

```
parse args (--ibus | --version | --help)
  ↓
init tracing (level từ env RUST_LOG)
  ↓
load config (best-effort, lỗi → dùng default)
  ↓
mở DBus session
  ↓
đăng ký EngineFactory với ibus-daemon
  ↓
chạy tokio event loop (current_thread runtime)
```

IBus interface `org.freedesktop.IBus.Factory` và `org.freedesktop.IBus.Engine` được implement qua `zbus` 5.

### 3.3. `crates/ibus-vie-cli/` — Dev tool

```
crates/ibus-vie-cli/
├── Cargo.toml
└── src/
    └── main.rs
```

Mục đích: gõ và xem output **không cần IBus chạy**. Cực kỳ quan trọng cho debug FSM.

**Ví dụ session:**

```
$ ibus-vie-cli --method telex
ibus-vie-cli (telex). Gõ vào, Ctrl+D để thoát.
> vieetj
việt
> tieengs
tiếng
> Vieetj Nam
Việt Nam
> chaof
chào
```

Khi user báo "gõ X không ra Y", maintainer chạy `ibus-vie-cli --method telex`, gõ X, so kết quả. Không cần restart ibus, không cần GUI. Đây là khác biệt lớn về tốc độ debug so với phải kiểm thử qua daemon.

Tùy chọn:

```
ibus-vie-cli --method telex                # tương tác
ibus-vie-cli --method telex --input vieetj # one-shot
ibus-vie-cli --method telex --trace        # in từng bước FSM
```

---

## 4. File `Cargo.toml` workspace (mẫu)

```toml
[workspace]
resolver = "2"
members = [
    "crates/ibus-vie-im",
    "crates/ibus-vie-engine",
    "crates/ibus-vie-cli",
]

[workspace.package]
version       = "0.1.0"
edition       = "2021"
rust-version  = "1.95"
license       = "GPL-3.0-or-later"
repository    = "https://github.com/IDev4life/ibus-vie"

[workspace.dependencies]
zbus              = "5"
tokio             = { version = "1", features = ["rt", "macros", "signal"] }
tracing           = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
serde             = { version = "1", features = ["derive"] }
toml              = "1.1"
clap              = { version = "4", features = ["derive"] }
thiserror         = "2"
insta             = "1"
```

---

## 5. Cấu hình build và install

### Makefile

```makefile
PREFIX      ?= /usr
LIBEXEC_DIR ?= $(PREFIX)/libexec
IBUS_DIR    ?= $(PREFIX)/share/ibus/component
BIN_DIR     ?= $(PREFIX)/bin

.PHONY: build install uninstall test fmt lint clean

build:
	cargo build --release

test:
	cargo test --workspace

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

check: fmt lint test

install: build
	install -Dm755 target/release/ibus-vie-engine $(DESTDIR)$(LIBEXEC_DIR)/ibus-engine-vie
	install -Dm755 target/release/ibus-vie-cli    $(DESTDIR)$(BIN_DIR)/ibus-vie-cli
	install -d $(DESTDIR)$(IBUS_DIR)
	sed 's|@LIBEXEC@|$(LIBEXEC_DIR)|g' data/vie.xml.in \
		> $(DESTDIR)$(IBUS_DIR)/vie.xml
	-ibus write-cache --system 2>/dev/null || true

uninstall:
	rm -f $(DESTDIR)$(LIBEXEC_DIR)/ibus-engine-vie
	rm -f $(DESTDIR)$(BIN_DIR)/ibus-vie-cli
	rm -f $(DESTDIR)$(IBUS_DIR)/vie.xml

clean:
	cargo clean

# Dev: install to user-local for testing without root
install-user: build
	install -d $(HOME)/.local/share/ibus/component
	sed 's|@LIBEXEC@|$(PWD)/target/release|g' data/vie.xml.in \
		> $(HOME)/.local/share/ibus/component/vie.xml
```

Makefile chỉ là wrapper mỏng. Toàn bộ logic build/test nằm trong cargo. Lý do giữ Makefile: dễ cho người đóng gói (Debian/RPM maintainers) — họ quen với `make install DESTDIR=...`.

---

## 6. Snapshot tests — cho cả người không biết Rust contribute

File `tests/snapshot/telex.txt`:

```
# Telex test cases. Format: <input><TAB><expected>
# Dòng bắt đầu # là comment. Dòng trống bỏ qua.

vieetj          việt
tieengs         tiếng
ddi             đi
chaof           chào
ddoongf         đồng
khoor           khở
nguwowif        người
Vieetj Nam      Việt Nam
```

Có một test Rust duy nhất đọc file này và check từng dòng:

```rust
// crates/ibus-vie-im/tests/snapshot_telex.rs
#[test]
fn telex_snapshot() {
    let text = include_str!("../../../tests/snapshot/telex.txt");
    let mut engine = ibus_vie_im::TelexEngine::new();
    for (lineno, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap().trim_end();
        if line.is_empty() { continue; }
        let (input, expected) = line.split_once('\t')
            .unwrap_or_else(|| panic!("line {}: missing tab", lineno + 1));
        let actual = engine.feed_str(input.trim());
        assert_eq!(actual, expected.trim(),
            "line {}: input {:?}", lineno + 1, input);
        engine.reset();
    }
}
```

Người báo lỗi gõ sai chỉ cần mở `tests/snapshot/telex.txt`, thêm dòng `chuỗi-gõ<TAB>kết-quả-đúng`, mở PR. Không cần viết Rust. Đây là "test as documentation".

---

## 7. Logging và debug

### 7.1. Logging

Crate engine dùng `tracing`. Mặc định in level `INFO` ra stderr. Bật debug:

```bash
RUST_LOG=ibus_vie=debug ibus restart
journalctl --user -f | grep ibus-vie
```

### 7.2. Các điểm log quan trọng

- Khởi tạo: phiên bản, DBus address, config path.
- Mỗi key event nhận được (level `trace`).
- Mỗi transition của FSM (level `debug`).
- Mỗi commit (level `debug`).
- Mọi lỗi DBus (level `warn` hoặc `error`).

### 7.3. Debug FSM mà không cần IBus

```bash
cargo run -p ibus-vie-cli -- --method telex --trace
```

→ in từng bước FSM, không phụ thuộc daemon. Đây là vòng feedback ngắn nhất khi develop logic gõ.

### 7.4. Debug DBus

```bash
busctl --user introspect org.freedesktop.IBus /org/freedesktop/IBus
dbus-monitor --session "interface='org.freedesktop.IBus.Engine'"
```

Khi engine "không phản hồi", phần lớn nguyên nhân là DBus — `busctl` và `dbus-monitor` là công cụ chẩn đoán nhanh nhất.

---

## 8. CI

GitHub Actions workflow (`.github/workflows/ci.yml`):

| Job    | Lệnh                                                       |
| ------ | ---------------------------------------------------------- |
| fmt    | `cargo fmt --all -- --check`                               |
| clippy | `cargo clippy --all-targets --all-features -- -D warnings` |
| test   | `cargo test --workspace`                                   |
| build  | `cargo build --release --workspace`                        |
| docs   | `cargo doc --no-deps --workspace`                          |

Chưa cần integration test với IBus thật trong CI ở giai đoạn đầu — phụ thuộc daemon, môi trường khó dựng. Test thật ở local.

---

## 9. Quy tắc khi thêm tính năng

Mỗi PR thêm tính năng nên trả lời được:

1. **Tính năng này thuộc crate nào?** Nếu là logic gõ → `ibus-vie-im`. Nếu là cách giao tiếp IBus → `ibus-vie-engine`. Nếu là dev tool → `ibus-vie-cli`.
2. **Có thêm test snapshot không?** Mọi thay đổi gõ phải có ít nhất 1 dòng mới trong `tests/snapshot/<method>.txt`.
3. **Có ảnh hưởng API public không?** Nếu có, cập nhật doc comment.
4. **Có thay đổi dependency không?** Tránh tăng dependency tree một cách không cần thiết.

---

## 10. Tóm tắt nhanh — "tôi muốn sửa X, mở file nào?"

| Muốn sửa                  | File                                                                       |
| ------------------------- | -------------------------------------------------------------------------- |
| Một từ Telex gõ sai       | `tests/snapshot/telex.txt` (thêm test) + `crates/ibus-vie-im/src/telex.rs` |
| Cách IBus gọi engine      | `crates/ibus-vie-engine/src/ibus/engine_impl.rs`                           |
| Config user               | `crates/ibus-vie-engine/src/config.rs`                                     |
| Tool gõ thử ở terminal    | `crates/ibus-vie-cli/src/main.rs`                                          |
| File XML đăng ký với IBus | `data/vie.xml.in`                                                          |
| Lệnh build / install      | `Makefile` + `Cargo.toml` (workspace)                                      |
