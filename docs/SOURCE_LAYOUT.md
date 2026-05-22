# SOURCE_LAYOUT — Cấu trúc source code

Tài liệu này mô tả **cách source được phân bổ**, ranh giới giữa các phần, và cách debug. Mục tiêu: ai mới đọc repo trong 10 phút phải biết "code FSM nằm ở đâu", "code DBus nằm ở đâu", "test gõ một từ thì sửa file nào".

Ngôn ngữ: **Rust** (edition 2021). Tổ chức: **Cargo workspace** với nhiều crate nhỏ.

[Chưa xác minh] Layout dưới đây là đề xuất thiết kế. Khi prototype Phase 0 chạy, có thể cần điều chỉnh — tài liệu này phải được update theo.

---

## 1. Sơ đồ tổng quát

```
ibus-vie/
├── Cargo.toml                       # [workspace] manifest
├── Cargo.lock
├── rust-toolchain.toml              # pin toolchain version
├── Makefile                         # wrapper: make build / install / test
├── README.md
├── CONTRIBUTING.md
├── LICENSE
│
├── crates/
│   ├── ibus-vie-im/                   # ① FSM thuần (no IBus, no DBus)
│   ├── ibus-vie-vi/                   # ② Quy tắc tiếng Việt (no I/O)
│   ├── ibus-vie-engine/               # ③ IBus binary (DBus + glue)
│   └── ibus-vie-cli/                  # ④ Dev tool: gõ ở terminal
│
├── data/
│   ├── vie.xml.in                # IBus component descriptor (template)
│   └── icons/
│       └── ibus-vie.svg
│
├── tests/
│   └── snapshot/                    # Test case dạng text (xem §6)
│       ├── telex.txt
│       ├── vni.txt
│       └── viqr.txt
│
├── benches/
│   └── fsm.rs                       # criterion benchmark
│
├── packaging/
│   ├── debian/
│   ├── rpm/ibus-vie.spec
│   └── arch/PKGBUILD
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

## 2. Bốn crate — ranh giới rõ

| #   | Crate             | Loại | Depend gì                    | Mục đích                                                        |
| --- | ----------------- | ---- | ---------------------------- | --------------------------------------------------------------- |
| ①   | `ibus-vie-im`     | lib  | chỉ `ibus-vie-vi`            | FSM Telex/VNI/VIQR thuần. Không I/O. Không IBus.                |
| ②   | `ibus-vie-vi`     | lib  | không depend gì              | Dữ liệu + luật âm tiết tiếng Việt. Const tables, đặt dấu thanh. |
| ③   | `ibus-vie-engine` | bin  | `ibus-vie-im` + `zbus` + ... | Binary chạy bởi `ibus-daemon`. Tất cả DBus/IBus glue ở đây.     |
| ④   | `ibus-vie-cli`    | bin  | `ibus-vie-im`                | Tool dev. Gõ vào terminal → in ra kết quả. Không cần IBus chạy. |

[Suy luận] Quy tắc vàng của layout này: **logic gõ không được biết IBus tồn tại**. `ibus-vie-im` phải compile và test được trên một máy không có IBus, không có DBus, không có GUI. Nếu một ngày con đường IBus engine không còn phù hợp (vd port sang Wayland input-method-v2 trực tiếp — xem `WAYLAND.md`), chỉ phải viết lại `ibus-vie-engine`; `ibus-vie-im` và `ibus-vie-vi` không đổi.

### Dependency graph

```
ibus-vie-cli  ────────┐
                    ▼
                ibus-vie-im
                    │
                    ▼
                ibus-vie-vi

ibus-vie-engine ──► ibus-vie-im
              ──► zbus, tokio, tracing, ...
```

[Suy luận] Không có cycle. `ibus-vie-vi` là "leaf" — không depend crate khác trong workspace. Điều này cho phép `cargo test -p ibus-vie-vi` chạy cực nhanh và độc lập.

---

## 3. Chi tiết từng crate

### 3.1. `crates/ibus-vie-vi/` — Quy tắc tiếng Việt

```
crates/ibus-vie-vi/
├── Cargo.toml
└── src/
    ├── lib.rs               # re-export public API
    ├── alphabet.rs          # const VOWELS, CONSONANTS, ...
    ├── tone.rs              # đặt dấu thanh trên âm tiết
    ├── syllable.rs          # struct Syllable { onset, nucleus, coda, tone }
    └── tables.rs            # bảng dữ liệu lớn (private)
```

**Public API mong đợi** (`lib.rs`):

```rust
pub mod alphabet;
pub mod tone;
pub mod syllable;

pub use syllable::Syllable;
pub use tone::{Tone, place_tone};
```

**Test:** mọi function public phải có ít nhất 1 unit test trong `#[cfg(test)] mod tests`.

### 3.2. `crates/ibus-vie-im/` — FSM gõ

```
crates/ibus-vie-im/
├── Cargo.toml
└── src/
    ├── lib.rs               # pub use các engine
    ├── action.rs            # enum Action, struct KeyEvent
    ├── engine.rs            # trait Engine
    ├── telex.rs             # TelexEngine impl Engine
    ├── vni.rs               # VniEngine impl Engine
    ├── viqr.rs              # ViqrEngine impl Engine
    └── buffer.rs            # preedit buffer + rewrite logic chung
```

**API cốt lõi** (`engine.rs`):

```rust
pub trait Engine {
    /// Đẩy một phím vào engine. Trả về Action mô tả việc cần làm.
    fn key(&mut self, ev: KeyEvent) -> Action;

    /// Reset preedit (khi focus chuyển, khi user gõ Esc, ...).
    fn reset(&mut self);

    /// Lấy preedit hiện tại để hiển thị.
    fn preedit(&self) -> &str;
}

pub enum Action {
    /// Cập nhật preedit, không commit gì.
    Update,
    /// Commit text này vào ứng dụng.
    Commit(String),
    /// Không xử lý phím, để IBus chuyển tiếp cho ứng dụng.
    PassThrough,
}
```

[Suy luận] Trait `Engine` chính là **điểm tách**. `ibus-vie-engine` chỉ làm việc với `dyn Engine` — không quan tâm Telex hay VNI. Thêm kiểu gõ mới = thêm một file impl `Engine`, không sửa engine binary.

### 3.3. `crates/ibus-vie-engine/` — IBus binary

```
crates/ibus-vie-engine/
├── Cargo.toml
└── src/
    ├── main.rs              # entry: parse CLI, init log, chạy event loop
    ├── ibus/
    │   ├── mod.rs
    │   ├── connection.rs    # mở session DBus tới ibus-daemon
    │   ├── factory.rs       # implement IBus EngineFactory interface
    │   ├── engine.rs        # implement IBus Engine interface (process_key_event, ...)
    │   └── proxy.rs         # các DBus proxy được sinh ra
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

[Chưa xác minh] Các DBus interface chính xác mà IBus expose (`org.freedesktop.IBus.Factory`, `org.freedesktop.IBus.Engine`) và signature methods phải được xác nhận qua `busctl introspect` hoặc IBus source tại Phase 0.

### 3.4. `crates/ibus-vie-cli/` — Dev tool

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

[Suy luận] Khi user báo "gõ X không ra Y", maintainer chạy `ibus-vie-cli --method telex`, gõ X, so kết quả. Không cần restart ibus, không cần GUI, log đầy đủ. Đây là khác biệt lớn về tốc độ debug so với phải kiểm thử qua daemon.

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
    "crates/ibus-vie-vi",
    "crates/ibus-vie-im",
    "crates/ibus-vie-engine",
    "crates/ibus-vie-cli",
]

[workspace.package]
version       = "0.1.0"
edition       = "2021"
rust-version  = "1.75"
license       = "GPL-3.0-or-later"          # [Chưa xác minh] chốt sau
repository    = "https://github.com/<owner>/ibus-vie"

[workspace.dependencies]
zbus              = "4"
tokio             = { version = "1", features = ["rt", "macros", "signal"] }
tracing           = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
serde             = { version = "1", features = ["derive"] }
toml              = "0.8"
clap              = { version = "4", features = ["derive"] }
thiserror         = "1"
insta             = "1"
```

[Chưa xác minh] Các version major number ở trên là dự kiến tại thời điểm viết. Khi tạo repo thật cần kiểm tra version mới nhất phù hợp.

---

## 5. Cấu hình build và install

### Makefile (mẫu)

```makefile
PREFIX      ?= /usr
LIBEXEC_DIR ?= $(PREFIX)/libexec
IBUS_DIR    ?= $(PREFIX)/share/ibus/component

.PHONY: build install uninstall test fmt lint clean

build:
	cargo build --release

test:
	cargo test --workspace

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

install: build
	install -Dm755 target/release/ibus-vie-engine $(DESTDIR)$(LIBEXEC_DIR)/ibus-engine-vie
	install -Dm755 target/release/ibus-vie-cli    $(DESTDIR)$(PREFIX)/bin/ibus-vie-cli
	sed 's|@LIBEXEC@|$(LIBEXEC_DIR)|g' data/vie.xml.in \
		> $(DESTDIR)$(IBUS_DIR)/vie.xml
	-ibus write-cache --system

uninstall:
	rm -f $(DESTDIR)$(LIBEXEC_DIR)/ibus-engine-vie
	rm -f $(DESTDIR)$(PREFIX)/bin/ibus-vie-cli
	rm -f $(DESTDIR)$(IBUS_DIR)/vie.xml

clean:
	cargo clean
```

[Suy luận] Makefile chỉ là wrapper mỏng. Toàn bộ logic build/test thực sự nằm trong cargo. Lý do giữ Makefile: dễ cho người đóng gói (Debian/RPM maintainers) — họ quen với `make install DESTDIR=...`.

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

[Suy luận] Người báo lỗi gõ sai chỉ cần mở `tests/snapshot/telex.txt`, thêm dòng `chuỗi-gõ<TAB>kết-quả-đúng`, mở PR. Không cần viết Rust. Đây là "test as documentation".

---

## 7. Logging và debug

### 7.1. Logging

Crate engine dùng `tracing`. Mặc định in level `INFO` ra stderr. Bật debug:

```bash
RUST_LOG=ibus_vie=debug ibus restart
journalctl --user -f -t ibus-daemon | grep ibus-vie
```

[Chưa xác minh] Cách `ibus-daemon` xử lý stderr của engine child process cần kiểm chứng — có thể log đi vào journald hoặc bị nuốt. Nếu bị nuốt, dùng file log fallback: `~/.cache/ibus-vie/engine.log`.

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

[Suy luận] Khi engine "không phản hồi", phần lớn nguyên nhân là DBus — `busctl` và `dbus-monitor` là công cụ chẩn đoán nhanh nhất.

---

## 8. CI đề xuất

[Suy đoán] GitHub Actions workflow (`.github/workflows/ci.yml`) đề xuất:

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

[Suy luận] Mỗi PR thêm tính năng nên trả lời được:

1. **Tính năng này thuộc crate nào?** Nếu là logic gõ → `ibus-vie-im`. Nếu là quy tắc tiếng Việt → `ibus-vie-vi`. Nếu là cách giao tiếp IBus → `ibus-vie-engine`. Nếu là dev tool → `ibus-vie-cli`.
2. **Có thêm test snapshot không?** Mọi thay đổi gõ phải có ít nhất 1 dòng mới trong `tests/snapshot/<method>.txt`.
3. **Có ảnh hưởng API public không?** Nếu có, cập nhật doc comment.
4. **Có thay đổi dependency không?** Tránh tăng dependency tree một cách không cần thiết.

---

## 10. Tóm tắt nhanh — "tôi muốn sửa X, mở file nào?"

| Muốn sửa                  | File                                                                       |
| ------------------------- | -------------------------------------------------------------------------- |
| Một từ Telex gõ sai       | `tests/snapshot/telex.txt` (thêm test) + `crates/ibus-vie-im/src/telex.rs` |
| Bảng phụ âm tiếng Việt    | `crates/ibus-vie-vi/src/alphabet.rs`                                       |
| Quy tắc đặt dấu           | `crates/ibus-vie-vi/src/tone.rs`                                           |
| Cách IBus gọi engine      | `crates/ibus-vie-engine/src/ibus/engine.rs`                                |
| Config user               | `crates/ibus-vie-engine/src/config.rs`                                     |
| Tool gõ thử ở terminal    | `crates/ibus-vie-cli/src/main.rs`                                          |
| File XML đăng ký với IBus | `data/vie.xml.in`                                                          |
| Lệnh build / install      | `Makefile` + `Cargo.toml` (workspace)                                      |
