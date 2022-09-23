# 雪還是學 ?

Just a ffi on top of my xuexi library to export it to swift (trying at least)

## Build

- Run the ./build-rust.sh
- Run the command

```
swift-bridge-cli create-package \
  --bridges-dir ./generated \
  --out-dir XuexiPackage \
  --ios target/aarch64-apple-ios/debug/libxuexi_swifty.a \
  --simulator target/universal-ios/debug/libxuexi_swifty.a \
  --macos target/universal-macos/debug/libxuexi_swifty.a \
  --name XuexiPackage
```

- Note some generated Swift code required the following @available attribute. Based on the code lint of vscode
add the following attribute at the top of a method

```
@available(macOS 10.15, *)
```