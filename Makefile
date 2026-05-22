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
	-ibus write-cache --system 2>/dev/null || true

clean:
	cargo clean

# Dev convenience: install to user-local for testing without root
install-user: build
	install -d $(HOME)/.local/share/ibus/component
	sed 's|@LIBEXEC@|$(PWD)/target/release|g' data/vie.xml.in \
		> $(HOME)/.local/share/ibus/component/vie.xml
	@echo "Installed user-local. Run: ibus restart"
