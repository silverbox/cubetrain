# インストール

## Cargo
[公式ページ](https://www.rust-lang.org/tools/install)に従う

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 必要ライブラリ

もしかしたら、最後のlibwebkit2gtk-4.0-devだけでよかったかも（依存ライブラリもインストールされた？）。

```bash
sudo apt install libglib2.0-* libcairo2-dev libsdl-pango-dev libgdk-pixbuf2.0-dev libgtk-3-dev libsoup-gnome2.4-dev libwebkit2gtk-4.0-dev libwebkit2gtk-4.0-37-gtk2
```

もしかしたら、`libatk1.0-0`も必要かも

# 参考にさせて頂いたページ

## web-view

https://crates.io/crates/web-view

## イベント処理

https://qiita.com/osanshouo/items/7966ecbd41bc3ce611dd