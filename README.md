# インストール

## Cargo

[公式ページ](https://www.rust-lang.org/tools/install)に従う

```bash
sudo apt install libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

```

## その他

```bash
cargo install cargo-make
cargo install cargo-generate
```

# seed

## プロジェクト新規作成

このプロジェクト作成時。このリポジトリをcloneしたならば不要。

```bash
cargo generate --git https://github.com/seed-rs/seed-quickstart.git --name cubetrain
cd cubetrain
```

## 起動

```bash
cargo make build_release
cargo make serve
```

## 参考にさせて頂いたページ

https://github.com/seed-rs/seed-quickstart



# yew

参考にさせていただいたページ通りにやったがコンパイルで失敗し諦める。

## インストール

```bash
rustup target add wasm32-unknown-unknown
rustup install nightly
rustup default nightly
```

## ビルド

```bash
cargo web start --target=wasm32-unknown-unknown --auto-reload
```

## 参考にさせて頂いたページ

https://nulab.com/ja/blog/nulab/rust-yew-webassembly-kanban-app/
https://dev.classmethod.jp/articles/yew-firststep/


# web-view

当初、web-viewで対応しようとしていたときの情報。  
Ubuntuにて、イベント処理が動かない（externalオブジェクトにアクセスした所で止まる）ので諦める。

## 必要ライブラリ

もしかしたら、最後のlibwebkit2gtk-4.0-devだけでよかったかも（依存ライブラリもインストールされた？）。

```bash
sudo apt install libglib2.0-* libcairo2-dev libsdl-pango-dev libgdk-pixbuf2.0-dev libgtk-3-dev libsoup-gnome2.4-dev libwebkit2gtk-4.0-dev
```

もしかしたら、`libatk1.0-0`も必要かも

## 参考にさせて頂いたページ

https://crates.io/crates/web-view
https://qiita.com/osanshouo/items/7966ecbd41bc3ce611dd
