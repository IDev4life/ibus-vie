# Changelog

## [1.0.0](https://github.com/IDev4life/ibus-vie/releases/tag/ibus-vie-v1.0.0) (2026-05-26)

### Features

* scaffold ibus-vie Rust workspace with full initial implementation ([19f6b5d](https://github.com/IDev4life/ibus-vie/commit/19f6b5d0d8d28864044c8929e8194b2c51c03abf))
* enforce no unsafe code across all workspace crates ([b5b352b](https://github.com/IDev4life/ibus-vie/commit/b5b352ba26968dd674062cd72a8ca66a4e9be5e8))
* **config:** persist method and input_mode to config.toml on UI switch ([dd4b086](https://github.com/IDev4life/ibus-vie/commit/dd4b08626f547aaa9eecd5f3e6a46d0bacf937ee))
* **ibus-vie-engine:** add runtime method switching via IBus property menu ([2d34e3c](https://github.com/IDev4life/ibus-vie/commit/2d34e3c5b3bb802f2b8dd3a0fb17e8d53db70293))
* **engine:** replace forward mode with popup mode using IBus lookup table ([f236015](https://github.com/IDev4life/ibus-vie/commit/f236015cd5de181259a4e32fb264a96bcd1f50c8))
* **engine:** use pure preedit mode and add underline config ([44088a4](https://github.com/IDev4life/ibus-vie/commit/44088a435932608b09118a887e8434dbe2396cd5))
* **makefile:** add setup-user target for one-command GNOME configuration ([d6724eb](https://github.com/IDev4life/ibus-vie/commit/d6724eb6aa5aefc2368cfdd1b88800dac3ed8a26))
* add version display + update check in IBus menu, and GitHub Pages apt repo ([9f8e298](https://github.com/IDev4life/ibus-vie/commit/9f8e298cb33901b8939fa6660d07630f7a6fe250))
* add .deb packaging via cargo-deb and GitHub Actions release workflow ([3bdbff7](https://github.com/IDev4life/ibus-vie/commit/3bdbff7fb349d4c4542ba08e30d1599ba3305b12))

### Bug Fixes

* **engine:** commit preedit on navigation keys before passing through ([e0b6157](https://github.com/IDev4life/ibus-vie/commit/e0b61576cea778430228be97224579fa2e057c90))
* **engine:** fix version label format and add GitHub link on click ([2ccc5ef](https://github.com/IDev4life/ibus-vie/commit/2ccc5ef1d6e750869be0e2f301934a3d2ca52f18))
* **engine:** restore IBus property menu with version display ([e00b862](https://github.com/IDev4life/ibus-vie/commit/e00b862c5d595e1b8ce645115dd28049ee5c36b0))
* **engine:** correct IBusLookupTable serialization ([12328e1](https://github.com/IDev4life/ibus-vie/commit/12328e17c4131e00059073fdf7dd7c68990871a0))
* **engine:** suppress auto-generated candidate number in popup lookup table ([584c6d2](https://github.com/IDev4life/ibus-vie/commit/584c6d20602389559d4379324595b3d7664fae4a))
* **makefile:** add --workspace flag to build target ([ff643dd](https://github.com/IDev4life/ibus-vie/commit/ff643dd5dce07ca5fd4aa8676d2d17d2ac88cdb2))
* **props:** rename preedit mode label to Underline for clarity ([47987f6](https://github.com/IDev4life/ibus-vie/commit/47987f684c9604c30b06be7f96dc439d5709ad5e))

### Code Refactoring

* remove VIQR, rework Engine trait and IBus signals ([e385d97](https://github.com/IDev4life/ibus-vie/commit/e385d9711f13b7c46702edf7e2026fe6c6fa02b3))
* replace ibus-vie-vi with vi crate from crates.io ([e6b2af9](https://github.com/IDev4life/ibus-vie/commit/e6b2af93d763c2d26e523b353f317a7a8328a8a4))
