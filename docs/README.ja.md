<p align="center">
  <img src="../src-tauri/icons/128x128@2x.png" width="128" alt="Risco のアイコン">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  <a href="../README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.gl.md">Galego</a> · <a href="README.fr.md">Français</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · 日本語 · <a href="README.ko.md">한국어</a> · <a href="README.zh-CN.md">简体中文</a> · <a href="README.tr.md">Türkçe</a> · <a href="README.az.md">Azərbaycanca</a> · <a href="README.hi.md">हिन्दी</a>
</p>

<h3 align="center">どんな Markdown にもハイライトを。ファイルには手を触れずに。</h3>

<p align="center">
  無料のオープンソース。Linux、Windows、macOS に対応。
  <br>
  <a href="https://github.com/CarlosCaoLopez/risco/releases/latest">ダウンロード&nbsp;&rsaquo;</a>
</p>

<br>

## インストール

[最新リリース](https://github.com/CarlosCaoLopez/risco/releases/latest)からお使いのシステム用のインストーラをダウンロードしてください。

| | |
|:--|:--|
| **macOS** | `.dmg` を開き、Risco をアプリケーションにドラッグします。 |
| **Windows** | `-setup.exe` を実行します。 |
| **Linux** | `.deb` か `.rpm` をインストールするか、`.AppImage` を実行します。 |

Homebrew でもインストールできます。

```sh
brew install --cask CarlosCaoLopez/tap/risco
```

Risco は Developer ID で署名され、Apple の公証を受けているので、ほかのアプリと同じように開けます。初回起動時、macOS がアクセシビリティへのアクセスを求めます。Risco がジェスチャーを認識するために必要です。インストーラはまだ署名されていないため、Windows では SmartScreen の警告が出ることがあります。その場合は *詳細情報 › 実行* を選んでください。

これで完了です。Risco はトレイに常駐し、コンピュータと一緒に起動し、すべての Obsidian 保管庫と VS Code に自動でプラグインをインストールします。

<br>

## 押して、なぞって、ハイライト。

<kbd>Ctrl</kbd>（macOS では <kbd>⌥</kbd>）を押しながら、Obsidian の閲覧ビュー、VS Code のプレビュー、または Orca のテキストをなぞります。蛍光ペンの音がして、ハイライトが残ります。

<!-- demo.gif -->

<br>

## まずは閲覧ビューへ。

ハイライトはレンダリングされたページの上で行うので、なぞる前に切り替えてください。

| | |
|:--|:--|
| **Obsidian** | 閲覧ビュー：<kbd>Ctrl</kbd>&thinsp;<kbd>E</kbd>（<kbd>⌘</kbd>&thinsp;<kbd>E</kbd>）、またはノート右上の本のアイコン。 |
| **VS Code** | Markdown プレビュー：<kbd>Ctrl</kbd>&thinsp;<kbd>Shift</kbd>&thinsp;<kbd>V</kbd>（<kbd>⇧</kbd>&thinsp;<kbd>⌘</kbd>&thinsp;<kbd>V</kbd>）、またはエディタの横に開くなら <kbd>Ctrl</kbd>&thinsp;<kbd>K</kbd> <kbd>V</kbd>。プレビューをクリックしてからなぞります。 |
| **Orca** | 切り替え不要：エディタで直接ハイライトできます。 |

Obsidian の編集ビューや VS Code のエディタでは、ジェスチャーは何もしません。

<br>

## すべてをひとつのジェスチャーで。

| | |
|:--|:--|
| <kbd>Ctrl</kbd> + ドラッグ | ハイライトする。 |
| ハイライトの上で <kbd>Ctrl</kbd> + ドラッグ | ハイライトを消す。 |
| <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> | 黄、ピンク、緑、青、紫。 |
| トレイの *最後を取り消す* | 直前のハイライトを取り消す。 |

<br>

## あなたの Markdown は、そのまま。

ハイライトはノート本体ではなく、各ノートの隣にある隠しファイルに保存されます。Obsidian のプラグインと VS Code の拡張機能がその場所に描画します。

Orca はプラグインを読み込めないため、そこでのハイライトは Orca が表示できる唯一のマークアップである `++テキスト++` としてノートに書き込まれます。

エディタに未保存の変更があるときは、何も書き込みません。

<br>

## 設計からプライベート。

アカウント不要。ネットワーク不要。アナリティクスなし。
Risco は完全にあなたのコンピュータ上で動作し、ノートが外に出ることはありません。

<br>

## 仕様

| | |
|:--|:--|
| **プラットフォーム** | Linux (X11)、Windows、macOS |
| **ビューア** | Obsidian、VS Code、Orca |
| **使用技術** | Rust、Tauri 2、TypeScript |
| **ネットワークアクセス** | なし |
| **価格** | 無料 |

<br>

## ソースからビルド

```sh
npm install
npm run tauri build
```

Linux では、通常の Tauri/WebKitGTK パッケージに加えて `libxtst` と appindicator ライブラリが必要です。

アプリは初回起動時にビューアのプラグインを自動でインストールします。
