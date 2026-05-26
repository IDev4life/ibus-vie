PREFIX      ?= /usr
LIBEXEC_DIR ?= $(PREFIX)/libexec
IBUS_DIR    ?= $(PREFIX)/share/ibus/component
BIN_DIR     ?= $(PREFIX)/bin

.PHONY: build install uninstall install-user setup-user deb test fmt lint clean

build:
	cargo build --release --workspace

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

# Build a .deb package (requires: cargo install cargo-deb)
deb:
	sed 's|@LIBEXEC@|/usr/libexec|g' data/vie.xml.in \
		> crates/ibus-vie-engine/data/vie.xml
	cargo build --release --workspace
	cargo deb -p ibus-vie-engine --no-build
	rm -f crates/ibus-vie-engine/data/vie.xml
	@echo "Package built: $$(ls target/debian/ibus-vie_*.deb)"

clean:
	cargo clean

# Dev convenience: install to user-local for testing without root
install-user: build
	install -d $(HOME)/.local/share/ibus/component
	sed 's|@LIBEXEC@|$(PWD)/target/release|g' data/vie.xml.in \
		> $(HOME)/.local/share/ibus/component/vie.xml
	@echo "Installed user-local. Run: ibus restart"

# Full user setup: install + configure GNOME input sources + restart IBus
# Replaces current input sources with: US keyboard + ibus-vie
setup-user: install-user
	ibus write-cache
	env DCONF_PROFILE=ibus dconf write /desktop/ibus/general/preload-engines "['ibus-vie']"
	gsettings set org.gnome.desktop.input-sources sources "[('xkb', 'us'), ('ibus', 'ibus-vie')]"
	ibus restart
	@echo "Done. Use Super+Space to switch between US and ibus-vie."
