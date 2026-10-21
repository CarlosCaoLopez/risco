<p align="center">
  <img src="../src-tauri/icons/128x128@2x.png" width="128" alt="Risco 아이콘">
</p>

<h1 align="center">Risco</h1>

<p align="center">
  <a href="../README.md">English</a> · <a href="README.es.md">Español</a> · <a href="README.gl.md">Galego</a> · <a href="README.fr.md">Français</a> · <a href="README.pt-BR.md">Português (Brasil)</a> · <a href="README.ja.md">日本語</a> · 한국어 · <a href="README.zh-CN.md">简体中文</a> · <a href="README.tr.md">Türkçe</a> · <a href="README.az.md">Azərbaycanca</a> · <a href="README.hi.md">हिन्दी</a>
</p>

<h3 align="center">어떤 Markdown이든 하이라이트하세요. 파일은 건드리지 않고.</h3>

<p align="center">
  무료 오픈 소스. Linux, Windows, macOS용.
  <br>
  <a href="https://github.com/CarlosCaoLopez/risco/releases/latest">다운로드&nbsp;&rsaquo;</a>
</p>

<br>

## 설치

[최신 릴리스](https://github.com/CarlosCaoLopez/risco/releases/latest)에서 사용 중인 시스템용 설치 파일을 내려받으세요.

| | |
|:--|:--|
| **macOS** | `.dmg`를 열고 Risco를 응용 프로그램으로 끌어다 놓으세요. |
| **Windows** | `-setup.exe`를 실행하세요. |
| **Linux** | `.deb` 또는 `.rpm`을 설치하거나 `.AppImage`를 실행하세요. |

Homebrew로도 설치할 수 있습니다.

```sh
brew install --cask CarlosCaoLopez/tap/risco
```

Risco는 Developer ID로 서명되고 Apple의 공증을 받았기 때문에 다른 앱처럼 바로 열립니다. 처음 실행하면 macOS가 손쉬운 사용 접근 권한을 요청하는데, Risco가 제스처를 인식하려면 필요합니다. 설치 파일이 아직 서명되지 않아 Windows에서 SmartScreen 경고가 나타날 수 있습니다. 그럴 땐 *추가 정보 › 실행*을 선택하세요.

이게 전부입니다. Risco는 트레이에 머물며 컴퓨터와 함께 시작하고, 모든 Obsidian 보관함과 VS Code에 플러그인을 알아서 설치합니다.

<br>

## 누르고. 긋고. 하이라이트.

<kbd>Ctrl</kbd>(macOS에서는 <kbd>⌥</kbd>)을 누른 채 Obsidian 읽기 보기, VS Code 미리보기 또는 Orca의 텍스트 위를 드래그하세요. 형광펜 소리가 나고, 하이라이트가 남습니다.

<!-- demo.gif -->

<br>

## 먼저 읽기 보기로.

하이라이트는 렌더링된 페이지에서 하므로, 드래그하기 전에 그쪽으로 전환하세요.

| | |
|:--|:--|
| **Obsidian** | 읽기 보기: <kbd>Ctrl</kbd>&thinsp;<kbd>E</kbd> (<kbd>⌘</kbd>&thinsp;<kbd>E</kbd>), 또는 노트 오른쪽 위의 책 아이콘. |
| **VS Code** | Markdown 미리보기: <kbd>Ctrl</kbd>&thinsp;<kbd>Shift</kbd>&thinsp;<kbd>V</kbd> (<kbd>⇧</kbd>&thinsp;<kbd>⌘</kbd>&thinsp;<kbd>V</kbd>), 또는 편집기 옆에 열려면 <kbd>Ctrl</kbd>&thinsp;<kbd>K</kbd> <kbd>V</kbd>. 미리보기를 클릭한 뒤 드래그하세요. |
| **Orca** | 전환할 필요 없음: 편집기에서 바로 하이라이트하세요. |

Obsidian 편집 보기나 VS Code 편집기에서는 제스처가 아무것도 하지 않습니다.

<br>

## 모든 것을 하나의 제스처로.

| | |
|:--|:--|
| <kbd>Ctrl</kbd> + 드래그 | 하이라이트하기. |
| 하이라이트 위에서 <kbd>Ctrl</kbd> + 드래그 | 지우기. |
| <kbd>Ctrl</kbd>&thinsp;<kbd>Alt</kbd>&thinsp;<kbd>1</kbd>–<kbd>5</kbd> | 노랑, 분홍, 초록, 파랑, 보라. |
| 트레이의 *마지막 실행 취소* | 마지막 하이라이트 취소. |

<br>

## 당신의 Markdown은 그대로.

하이라이트는 노트 안이 아니라 각 노트 옆의 숨김 파일에 저장됩니다. Obsidian 플러그인과 VS Code 확장이 제자리에 그려 줍니다.

Orca는 플러그인을 불러올 수 없어서, 거기서는 Orca가 렌더링하는 유일한 마크업인 `++텍스트++` 형태로 노트에 들어갑니다.

편집기에 저장되지 않은 변경 사항이 있으면 아무것도 쓰지 않습니다.

<br>

## 처음부터 프라이빗하게.

계정 없음. 네트워크 없음. 분석 없음.
Risco는 전적으로 당신의 컴퓨터에서 실행되며, 노트는 절대 밖으로 나가지 않습니다.

<br>

## 기술 사양

| | |
|:--|:--|
| **플랫폼** | Linux (X11), Windows, macOS |
| **뷰어** | Obsidian, VS Code, Orca |
| **사용 기술** | Rust, Tauri 2, TypeScript |
| **네트워크 접근** | 없음 |
| **가격** | 무료 |

<br>

## 소스에서 빌드

```sh
npm install
npm run tauri build
```

Linux에서는 일반적인 Tauri/WebKitGTK 패키지와 함께 `libxtst`, appindicator 라이브러리도 필요합니다.

앱은 처음 실행할 때 뷰어 플러그인을 스스로 설치합니다.
