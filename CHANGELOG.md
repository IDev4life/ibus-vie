# Changelog

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
