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

# 各種参考にさせていただいたページ

## Rustの技術

### 公式ページ

- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/title-page.html)
- [Rust by Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/index.html)

### 個人ページ（公式だったらごめんなさい）

#### ソート実行に関して

https://uma0317.github.io/rust-cookbook-ja/algorithms/sorting.html

## 3Dオブジェクトの描画

### 2Dへの投影に関して

#### 3D座標変換知識
https://sbfl.net/blog/2016/09/05/webgl2-tutorial-3d-knowledge/
http://www.cgg.cs.tsukuba.ac.jp/~endo/lecture/2020/cgbasics/03/03.pdf

#### ワールド座標からカメラ座標への変換
https://mem-archive.com/2018/02/17/post-74/

#### カメラY軸に関して
https://yttm-work.jp/gmpg/gmpg_0003.html

#### 透視変換の行列式
http://kondolab.org/archive/2010/research/cadcgtext/Chap5/Chap504.html

### 隠線処理

http://www.myu.ac.jp/~makanae/CG/cg1_14.htm

### オイラー角

https://www.sky-engin.jp/blog/eulerian-angles/

# 以降、当時トライしたが導入断念したフロー

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
