# SAR Auto Splitter

LiveSplit One compatible auto splitter for [SourceAutoRecord](https://sar.portal2.sr).

## Setup

```sh
rustup target add wasm32-unknown-unknown
```

## Building

```sh
cargo build --release
```

## Installation

Latest release is included in LiveSplit natively. See [LiveSplit.AutoSplitters](https://github.com/LiveSplit/LiveSplit.AutoSplitters/blob/master/LiveSplit.AutoSplitters.xml)

Wasm file is at: `target/wasm32-unknown-unknown/release/sar_auto_splitter.wasm`. Add to layout (Edit Layout -> Add -> Control -> Auto Splitting Runtime)

## Debugging

Get [asr-debugger](https://github.com/LiveSplit/asr-debugger).
