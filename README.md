# ルービック・キューブ訓練アプリ

## 実装済み機能

- 標準回転
  - レイヤ、軸単位での回転
  - 軸単位での回転
  - 揃い判定
- 標準ビュー（固定視点からの参照）
- 自動シャッフル
- サブビューで反対側参照

## 実装予定機能

- キー操作での回転
- サブビューで各種視点参照
- 各種ステップ毎に揃える対象以外の面を黒塗り
- 任意のステップにてキューブの状態を保持し、復帰出来る

# セットアップ１（rust,cargo）

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

# セットアップ２（seed）

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


# セットアップ３（vue）

## vue開発準備

コンテナ起動してvueのコンテナへログイン。

```bash
sudo docker-compose build
sudo docker-compose up
sudo docker exec -it cubetrain-vuefront bash
```

プロジェクト作成

```bash
yarn global add @vue/cli
vue create cubetrain
```

## seedとの連携

### seedコンパイル

```
cd wasm
cargo make build_release
cd ..
```

### vue側へコピー

vue側のコンパイルに必要になるので、`package.js`、`package_bg.wasm`をvue側へコピー。wasmとjsのインターフェース部が変わった時は再処理する。

```
cp wasm/pkg/package.js vue/cubetrain/src/wasm/package.js
cp wasm/pkg/package_bg.wasm vue/cubetrain/src/wasm/package_bg.wasm
```

以下の行をコピー先の`package.js`の最初に追記

```
/* eslint-disable */
```

### アクセス

package_bg.wasmはnginx経由で取得する事になるので、`docker-compose up`実行後、以下のアドレスにアクセス。  
http://localhost:8880


# 各種参考にさせていただいたページ

## ルービック・キューブのルールに関して

- [3x3x3 回転記号](https://tribox.com/3x3x3/solution/notation/)

## Rustの技術

### 公式ページ

- [The Rust Programming Language 日本語版](https://doc.rust-jp.rs/book-ja/title-page.html)
- [Rust by Example 日本語版](https://doc.rust-jp.rs/rust-by-example-ja/index.html)

### 個人ページ（公式だったらごめんなさい）

#### ソート実行に関して

- [ベクタのソート](https://uma0317.github.io/rust-cookbook-ja/algorithms/sorting.html)

## 3Dオブジェクトの描画

### 2Dへの投影に関して

#### 3D座標変換知識

- [WebGL2入門 3D知識編](https://sbfl.net/blog/2016/09/05/webgl2-tutorial-3d-knowledge/)
- [コンピュータグラフィックス基礎](http://www.cgg.cs.tsukuba.ac.jp/~endo/lecture/2020/cgbasics/03/03.pdf)

#### ワールド座標からカメラ座標への変換

- [カメラ外部パラメータとは](https://mem-archive.com/2018/02/17/post-74/)

#### カメラY軸に関して

- [ビュー座標変換](https://yttm-work.jp/gmpg/gmpg_0003.html)

#### 透視変換の行列式

- [5.4　透視図の数学](http://kondolab.org/archive/2010/research/cadcgtext/Chap5/Chap504.html)

### 隠線処理

- [隠線処理の手法](http://www.myu.ac.jp/~makanae/CG/cg1_14.htm)

### オイラー角

- [スカイ技術研究所ブログ](https://www.sky-engin.jp/blog/eulerian-angles/)

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
