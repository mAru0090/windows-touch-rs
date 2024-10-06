# windows-touch-rs
## 概要
このプログラムは、Unix/Linux系の'touch'コマンドをWindowsで使えるようにしたものですー
## 目次
- [インストール方法](#インストール方法)
- [使い方](#使い方)
- [機能](#機能)
- [ライセンス](#ライセンス)
- [貢献](#貢献)
- [作者](#作者)
- [参考リンク](#参考リンク)

## インストール方法
# ソースコードからビルドする場合(ソースコードからビルドする場合Rustコンパイラが必要です)
```
  git clone https://github.com/tmaru0090/windows-touch-rs
  cd windows-touch-rs
  cargo build --release
```
# リリースチャンネルをクローンする場合
```
  git clone https://github.com/tmaru0090/windows-touch-rs
  cd windows-touch-rs
  git checkout tags/[バージョン]
```
## 使い方
```
  Usage: touch.exe [OPTIONS] [FILE]...

  Arguments:
    [FILE]...

  Options:
    -a, --access
    -m, --modify
    -c, --no-create
    -d, --date <DATE>
    -t, --time <TIME>
    -r, --reference <REFERENCE>
    -h, --help                   Print help
```
## 機能
Unix/Linuxの'touch'コマンドと同じように使えますー
