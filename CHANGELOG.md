# Changelog

## [2.0.2](https://github.com/IDev4life/ibus-vie/compare/ibus-vie-v2.0.1...ibus-vie-v2.0.2) (2026-05-26)


### Bug Fixes

* **ci:** generate vie.xml in crates/ibus-vie-engine/data/ for cargo-deb ([d8789ea](https://github.com/IDev4life/ibus-vie/commit/d8789ea27038822817e5a11134fe5941584eed0f))

## [2.0.1](https://github.com/IDev4life/ibus-vie/compare/ibus-vie-v2.0.0...ibus-vie-v2.0.1) (2026-05-26)


### Bug Fixes

* **ci:** use -p flag instead of --bin to build workspace packages ([4e0af02](https://github.com/IDev4life/ibus-vie/commit/4e0af026b89eb3d573779ea20b485899f667cd84))

## [2.0.0](https://github.com/IDev4life/ibus-vie/compare/ibus-vie-v1.1.0...ibus-vie-v2.0.0) (2026-05-26)


### ⚠ BREAKING CHANGES

* ibus-vie-vi crate removed from the workspace
* ViqrEngine removed from public API of ibus-vie-im

### Features

* add .deb packaging via cargo-deb and GitHub Actions release workflow ([3bdbff7](https://github.com/IDev4life/ibus-vie/commit/3bdbff7fb349d4c4542ba08e30d1599ba3305b12))
* add forward-key input mode ([0edfb4f](https://github.com/IDev4life/ibus-vie/commit/0edfb4fa11a5f772718cc294ffffc32e7f2f7348))
* add version display + update check in IBus menu, and GitHub Pages apt repo ([9f8e298](https://github.com/IDev4life/ibus-vie/commit/9f8e298cb33901b8939fa6660d07630f7a6fe250))
* **config:** persist method and input_mode to config.toml on UI switch ([dd4b086](https://github.com/IDev4life/ibus-vie/commit/dd4b08626f547aaa9eecd5f3e6a46d0bacf937ee))
* **engine:** replace forward mode with popup mode using IBus lookup table ([f236015](https://github.com/IDev4life/ibus-vie/commit/f236015cd5de181259a4e32fb264a96bcd1f50c8))
* **engine:** use pure preedit mode and add underline config ([44088a4](https://github.com/IDev4life/ibus-vie/commit/44088a435932608b09118a887e8434dbe2396cd5))
* **ibus-vie-engine:** add runtime method switching via IBus property menu ([2d34e3c](https://github.com/IDev4life/ibus-vie/commit/2d34e3c5b3bb802f2b8dd3a0fb17e8d53db70293))
* **makefile:** add setup-user target for one-command GNOME configuration ([d6724eb](https://github.com/IDev4life/ibus-vie/commit/d6724eb6aa5aefc2368cfdd1b88800dac3ed8a26))


### Bug Fixes

* **engine:** commit preedit on navigation keys before passing through ([e0b6157](https://github.com/IDev4life/ibus-vie/commit/e0b61576cea778430228be97224579fa2e057c90))
* **engine:** correct IBusLookupTable serialization — remove v-boxing from primitive fields ([12328e1](https://github.com/IDev4life/ibus-vie/commit/12328e17c4131e00059073fdf7dd7c68990871a0))
* **engine:** fix version label format and add GitHub link on click ([2ccc5ef](https://github.com/IDev4life/ibus-vie/commit/2ccc5ef1d6e750869be0e2f301934a3d2ca52f18))
* **engine:** restore IBus property menu with version display ([e00b862](https://github.com/IDev4life/ibus-vie/commit/e00b862c5d595e1b8ce645115dd28049ee5c36b0))
* **engine:** suppress auto-generated candidate number in popup lookup table ([584c6d2](https://github.com/IDev4life/ibus-vie/commit/584c6d20602389559d4379324595b3d7664fae4a))
* **forward-key:** filter IBUS_FORWARD_MASK and use delta forwarding ([d6d7130](https://github.com/IDev4life/ibus-vie/commit/d6d71306ffa60b400f03991ab93c3e36f3bc617f))
* **makefile:** add --workspace flag to build target ([ff643dd](https://github.com/IDev4life/ibus-vie/commit/ff643dd5dce07ca5fd4aa8676d2d17d2ac88cdb2))
* **props:** rename preedit mode label to Underline for clarity ([47987f6](https://github.com/IDev4life/ibus-vie/commit/47987f684c9604c30b06be7f96dc439d5709ad5e))


### Code Refactoring

* remove VIQR, rework Engine trait and IBus signals ([e385d97](https://github.com/IDev4life/ibus-vie/commit/e385d9711f13b7c46702edf7e2026fe6c6fa02b3))
* replace ibus-vie-vi with vi crate from crates.io ([e6b2af9](https://github.com/IDev4life/ibus-vie/commit/e6b2af93d763c2d26e523b353f317a7a8328a8a4))

## [1.1.0](https://github.com/IDev4life/ibus-vie/compare/ibus-vie-v1.0.0...ibus-vie-v1.1.0) (2026-05-22)


### Features

* enforce no unsafe code across all workspace crates ([b5b352b](https://github.com/IDev4life/ibus-vie/commit/b5b352ba26968dd674062cd72a8ca66a4e9be5e8))
* scaffold ibus-vie Rust workspace with full initial implementation ([19f6b5d](https://github.com/IDev4life/ibus-vie/commit/19f6b5d0d8d28864044c8929e8194b2c51c03abf))


### Bug Fixes

* add --workspace to cargo-deny and trim unused licenses ([b14822f](https://github.com/IDev4life/ibus-vie/commit/b14822f0af231a3738e9b0e86eab6f618571a558))
* add root package manifest for release-please compatibility ([178c3a2](https://github.com/IDev4life/ibus-vie/commit/178c3a2062542d612499391f043165329686b5ed))
* **ci:** address second round of workflow review findings ([7dd85b5](https://github.com/IDev4life/ibus-vie/commit/7dd85b524ba795b437bfcf9c2ccc3731f939780b))
* **ci:** address workflow review findings ([58a9a21](https://github.com/IDev4life/ibus-vie/commit/58a9a210918cb190edb913973c990de5365d442d))
* **ci:** set CodeQL build_mode to none for Rust (autobuild unsupported) ([e1144bb](https://github.com/IDev4life/ibus-vie/commit/e1144bb95380ee3bcf3a92754171e6ec993148dd))
* **ci:** strip v prefix from tar.gz filename to match deb naming ([9ea4f59](https://github.com/IDev4life/ibus-vie/commit/9ea4f59de10064449501642f80f7039de76cc6c6))
* **ci:** use Debian arch names (amd64/arm64) for tar.gz artifacts ([9a802d0](https://github.com/IDev4life/ibus-vie/commit/9a802d06601ead634fca848abe31231cc456ce30))
* **ci:** use shell vars $ARCHIVE and $DEB in upload step ([5939755](https://github.com/IDev4life/ibus-vie/commit/59397551d8c03e34101c5504f5385dd8ed02e80e))
* **ci:** use short arch names in tar.gz (x86_64-linux, aarch64-linux) ([1e3923a](https://github.com/IDev4life/ibus-vie/commit/1e3923a41d592761e2aac9e6e22508614496f921))
* **data:** correct repository URL in vie.xml.in ([13f2f1f](https://github.com/IDev4life/ibus-vie/commit/13f2f1f60f0f11d0633c8cbf858925c862d2d09d))
* mark all crates publish=false to satisfy cargo-deny wildcard check ([e8706ce](https://github.com/IDev4life/ibus-vie/commit/e8706cedbbb50b29ce2928d19763d90ff142657d))
* remove deprecated vulnerability key from deny.toml ([64d9375](https://github.com/IDev4life/ibus-vie/commit/64d937565577b58c9221d64c6a375e6fa73e7c34))
* update deny.toml to cargo-deny v2 advisories syntax ([026ea5f](https://github.com/IDev4life/ibus-vie/commit/026ea5f253dcb07578e37726c1d2011edf21e3a1))
* use explicit version in each crate for release-please compatibility ([8e1db75](https://github.com/IDev4life/ibus-vie/commit/8e1db7593c54df1be9507c199823b6109eb29236))
