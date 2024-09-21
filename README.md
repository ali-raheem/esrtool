# ESRtool

Port of the PS2 Homebrew tool [esr-disc-patcher-cli](https://github.com/ali-raheem/esrtool-legacy). Based on [Tatsh's original work posted on PSX-Scene](https://web.archive.org/web/20150919031500/http://psx-scene.com/forums/f164/esr-disc-patcher-linux-mac-qt4-port-60096/).

These older versions contain several bugs, at least one of which is potentially exploitable. This tool is safer in that regard at least.

# Installation

* Either you can clone this repo and build in the usual way with `cargo build --release`. Or use `cargo install esrtool` to install it from [crates.io](https://crates.io/crates/esrtool).
* There is a GUI version available, just build `gui` feature: `cargo build --features gui --release`

# Usage

## Patching

Patching is done in place to maintain drop in compatibility with exisitng tools. But it is **lossy** maintain a clean rip of your discs for archive.

`esrtool p PATH_TO_ISO`

## Unpatching

`esrtool u PATH_TO_ISO`

**NOTE:** Unpatching is not clean, do not use it to store copies of the ISO. Instead maintain a clean one. Data is lost during the patching process. Infact many other tools I've tested drop junk into the file including potentially private environment data.

# License

MIT Licensed to fit with similar software. Also note notice in other ESR patching tools below.